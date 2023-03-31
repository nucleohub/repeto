use crate::predict::alignment::Score;

use super::algo::{BestDirectionTracer, GapTracer, Tracer};
use super::algo::FullScan;
use super::storage::Storage;
use super::super::super::{Alignable, local};
use super::super::super::alignment;
use super::super::super::scoring::ScoringScheme;
use super::traceback::TraceMat;

struct Tracers<S: Storage, T: TraceMat> {
    pub storage: S,
    pub tracemat: T,
}


impl<S: Storage, T: TraceMat> BestDirectionTracer for Tracers<S, T> {
    #[inline(always)]
    fn gap_row(&mut self, row: usize, col: usize, score: Score) {
        self.storage.gap_row(row, col, score);
        self.tracemat.gap_row(row, col, score);
    }

    #[inline(always)]
    fn gap_col(&mut self, row: usize, col: usize, score: Score) {
        self.storage.gap_col(row, col, score);
        self.tracemat.gap_col(row, col, score);
    }

    #[inline(always)]
    fn equivalent(&mut self, row: usize, col: usize, score: Score) {
        self.storage.equivalent(row, col, score);
        self.tracemat.equivalent(row, col, score);
    }

    #[inline(always)]
    fn none(&mut self, row: usize, col: usize) {
        self.storage.none(row, col);
        self.tracemat.none(row, col);
    }
}

impl<S: Storage, T: TraceMat> GapTracer for Tracers<S, T> {
    #[inline(always)]
    fn row_gap_open(&mut self, row: usize, col: usize, score: Score) {
        self.storage.row_gap_open(row, col, score);
        self.tracemat.row_gap_open(row, col, score);
    }

    #[inline(always)]
    fn row_gap_extend(&mut self, row: usize, col: usize, score: Score) {
        self.storage.row_gap_extend(row, col, score);
        self.tracemat.row_gap_extend(row, col, score);
    }

    #[inline(always)]
    fn col_gap_open(&mut self, row: usize, col: usize, score: Score) {
        self.storage.col_gap_open(row, col, score);
        self.tracemat.col_gap_open(row, col, score);
    }

    #[inline(always)]
    fn col_gap_extend(&mut self, row: usize, col: usize, score: Score) {
        self.storage.col_gap_extend(row, col, score);
        self.tracemat.col_gap_extend(row, col, score);
    }
}

impl<S: Storage, T: TraceMat> Tracer for Tracers<S, T> {
    #[inline(always)]
    fn first_col_start(&mut self) {
        self.storage.first_col_start();
        self.tracemat.first_col_start();
    }

    #[inline(always)]
    fn first_col_end(&mut self) {
        self.storage.first_col_end();
        self.tracemat.first_col_end();
    }

    #[inline(always)]
    fn col_start(&mut self, col: usize) {
        self.storage.col_start(col);
        self.tracemat.col_start(col);
    }

    #[inline(always)]
    fn col_end(&mut self, col: usize) {
        self.storage.col_end(col);
        self.tracemat.col_end(col);
    }
}


pub struct Engine<S: Storage, T: TraceMat, SF: ScoringScheme> {
    algo: FullScan,
    scoring: SF,
    tracers: Tracers<S, T>,
}


impl<S: Storage, T: TraceMat, SF: ScoringScheme> Engine<S, T, SF> {
    pub fn new(storage: S, tracemat: T, scoring: SF) -> Self {
        let algo = FullScan::new(0);
        Self { algo, scoring, tracers: Tracers { storage, tracemat } }
    }
}

impl<S1: Alignable, S2: Alignable, S: Storage, T: TraceMat, SF: ScoringScheme> local::Aligner<S1, S2, SF> for Engine<S, T, SF> {
    fn reconfigure(&mut self, scoring: SF) {
        self.scoring = scoring
    }

    fn align(&mut self, seq1: &S1, seq2: &S2) -> Result<local::Alignment, ()> {
        if seq1.len() == 0 || seq2.len() == 0 {
            return Err(());
        }

        self.tracers.storage.reset(seq1.len(), seq2.len());
        self.tracers.tracemat.reset(seq1.len(), seq2.len());

        self.algo.align(seq1, seq2, &mut self.scoring, &mut self.tracers);

        self.tracers.storage.finalize().into_iter().map(|x| {
            let trace = self.tracers.tracemat.trace(x.row, x.col).unwrap();
            debug_assert_eq!(trace.seq1.end, x.row + 1);
            debug_assert_eq!(trace.seq2.end, x.col + 1);
            let ops = alignment::utils::disambiguate(
                trace.ops, &self.scoring,
                seq1, trace.seq1.start,
                seq2, trace.seq2.start,
            );

            local::Alignment {
                score: x.score,
                steps: ops,
                seq1: trace.seq1,
                seq2: trace.seq2,
            }
        }).max_by_key(|x| x.score).ok_or(())
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use super::super::{storage, traceback};
    use super::super::super::super::scoring;

    #[test]
    fn sw_local_best() {
        let mut engine = Engine::new(
            storage::Best::new(),
            traceback::TraceMatrix::new(),
            scoring::default(),
        );

        local::test_suite::best::test_all(&mut engine);
    }
}
