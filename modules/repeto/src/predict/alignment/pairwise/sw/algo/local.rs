use std::cmp::max;

use crate::predict::alignment::pairwise::Aligner;

use super::{Alignable, ScoringScheme, Storage, Tracer};
use super::super::Score;
use super::super::traceback::Trace;

// Optimal alignments in linear space: https://doi.org/10.1093/bioinformatics/4.1.11

// For the alignment of A and B of length i and j:
// C(i, j) = max score
// D(i, j) = max score where A(i) is deleted
// I(i, j) = max score where B(i) is deleted

// Dynamic programming equations:
// C(i,j) = max {
//      D(i,j)
//      I(i,j)
//      C(i-1,j-1) + equiv(ai, bj)
//      0
// }
// D(i,j) = max {
//      D(i-1,j) + gap-extend
//      C(i-1,j) + gap-open + gap-extend
// }
// I(i,j) = {
//      I(i,j-1) + gap-extend
//      C(i,j-1) + gap-open + gap-extend
// }

// Optimization:
// C:
//      scores(i) = current C column
//      diagonal = C(i-1,j-1)
//      left = C(i-1, j)
// I:
//      gapcol(i) = current I column (horizontal seq2 gaps)
// D:
//      gaprow = current D(i-1) value (vertical seq1 gaps)

pub struct FullScan {
    left: Score,
    diagonal: Score,
    scores: Vec<Score>,
    gapcol: Vec<Score>,
    gaprow: Score,
}

impl FullScan {
    pub fn new(capacity: usize) -> Self {
        Self {
            left: 0,
            diagonal: 0,
            scores: Vec::with_capacity(capacity),
            gapcol: Vec::with_capacity(capacity),
            gaprow: 0,
        }
    }

    pub fn reserve(&mut self, capacity: usize) {
        debug_assert!(self.scores.capacity() == self.gapcol.capacity());
        if self.scores.capacity() < capacity {
            let diff = self.scores.capacity() - capacity;
            self.scores.reserve(diff);
            self.gapcol.reserve(diff);
        }
    }

    fn solve_first_col(&mut self, seq1: &impl Alignable, seq2: &impl Alignable,
                       scorer: &mut impl ScoringScheme, tracer: &mut impl Tracer, storage: &mut impl Storage) {
        let s2 = seq2.at(0);
        // Initialize gap-col
        // (required in later recursion, set to 0 for faster initialization)
        self.gapcol.clear();
        self.gapcol.resize(seq1.len(), 0);

        // Initialize scores, the first column - only equiv are allowed
        // (assuming that gap-row scores will be always <= 0)
        self.scores.clear();
        // TODO: verify that extend is faster
        self.scores.extend((0..seq1.len()).map(|row| {
            let equiv = scorer.score(row, seq1.at(row), 0, s2);
            if equiv > 0 {
                tracer.equivalent(row, 0);
                storage.equivalent(row, 0, equiv);
                equiv
            } else {
                tracer.restart(row, 0);
                storage.restart(row, 0);
                0
            }
        }))
    }

    pub fn align(&mut self, seq1: &impl Alignable, seq2: &impl Alignable,
                 scorer: &mut impl ScoringScheme, tracer: &mut impl Tracer, storage: &mut impl Storage) {
        self.solve_first_col(seq1, seq2, scorer, tracer, storage);
        for col in 1..seq1.len() {
            // First element
            self.gaprow = 0;
            self.diagonal = self.scores[0];
            let equiv = scorer.score(0, seq1.at(0), col, seq2.at(col));
            self.scores[0] = if equiv > 0 {
                tracer.equivalent(0, col);
                storage.equivalent(0, col, equiv);
                equiv
            } else {
                tracer.restart(0, col);
                storage.equivalent(0, col);
                0
            };

            for row in 1..seq2.len() {
                // Vertical/row gaps
                self.gaprow = max(
                    self.gaprow,
                    self.scores[row - 1] + scorer.seq1_gap_open(row - 1),
                ) + scorer.seq1_gap_extend(row);

                // Horizontal/column gaps
                self.gapcol[row] = max(
                    self.gapcol[row],
                    self.scores[row] + scorer.seq2_gap_open(row),
                ) + scorer.seq2_gap_extend(col);

                // Best scores
                self.left = scorer[row];
                let equiv = scorer.score(row, seq1.at(row), col, seq2.at(col));
                scores[row] = if equiv > self.gaprow && equiv > self.gapcol[row] && equiv > 0 {
                    tracer.equivalent(row, col);
                    storage.equivalent(row, col, equiv);
                    equiv
                } else if self.gapcol[row] > self.gaprow && self.gapcol[row] > 0 {
                    tracer.gap_col(row, col);
                    storage.gap_col(row, col, self.gapcol[row]);
                    self.gapcol[row]
                } else if self.gaprow > 0 {
                    tracer.gap_row(row, col);
                    storage.gap_row(row, col, self.gaprow);
                    self.gaprow
                } else {
                    tracer.restart(row, col);
                    storage.restart(row, col);
                    0
                };

                self.diagonal = self.left;
            }
        }
    }
}
