use super::super::scoring::symbols::SubType;

use super::super::traceback::Trace;
use super::super::Score;
use super::{Alignable, Aligner, ScoringFunc, Storage, Tracer};

pub struct DynProg {
    pub cache: Vec<Score>,
}

impl DynProg {
    pub fn new(capacity: usize) -> Self {
        Self {
            cache: Vec::with_capacity(capacity),
        }
    }

    // fn solve_first_row(&mut self, col: usize, seq1: &impl Alignable, seq2: &impl Alignable,
    //                    scorer: &mut impl ScoringFunc, tracer: &mut impl Tracer, storage: &mut impl Storage)
    // {
    //     // The first row is special, only gap-col or matches are allowed
    //     let (s1, s2) = (seq1.at(0), seq2.at(col));
    //
    //     // Match - mismatch ?
    //     let diagonal = scorer.score(0, s1, col, s2);
    //     // Gap col? TODO: fix affine gap penalties
    //     let gapcol = self.cache[0] - scorer.gapcol(0, 1);
    //
    //     if gapcol > 0 && gapcol > diagonal.0 {
    //         tracer.gap_col(0, col);
    //         self.cache[0] = gapcol;
    //         // storage.step(0, col, result.0, Trace::GapCol);
    //     } else if diagonal.0 > 0 {
    //         match diagonal.1 {
    //             SubType::Match => {
    //                 tracer.matched(0, col);
    //                 // storage.step(0, col, result.0, Trace::Matched);
    //             }
    //             SubType::Mismatch => {
    //                 tracer.mismatched(0, col);
    //                 // storage.step(0, col, result.0, Trace::Mismatch);
    //             }
    //         }
    //         self.cache[0] = diagonal.0;
    //     } else {
    //         self.cache[0] = 0;
    //     }
    // }

    // fn solve_first_col(&mut self, seq1: &impl Alignable, seq2: &impl Alignable,
    //                    scorer: &mut impl ScoringFunc, tracer: &mut impl Tracer, storage: &mut impl Storage)
    // {
    //     // Prepare the cache
    //     self.cache.clear();
    //     if self.cache.capacity() < seq1.len() {
    //         self.cache.reserve(self.cache.capacity() - seq1.len());
    //     }
    //
    //     // The very first element is special - it can be only the match or nothing
    //     let (s1, s2) = (seq1.at(0), seq2.at(0));
    //     let option = scorer.score(0, s1, 0, s2);
    //     let result = if option.0 > 0 { (option.0, option.1.into()) } else { (0, Trace::None) };
    //
    //     self.cache.push(result.0);
    //     tracer.update(0, 0, result.1);
    //     storage.step(0, 0, result.0, result.1);
    //     //------------------------------------------------------------------------------------------
    //     // Other elements are less special - they can only be gap-row or match
    //     for row in 1..self.cache.len() {
    //         let mut result = (0, Trace::None);
    //
    //         // Match?
    //         let (s1, s2) = (seq1.at(row), seq2.at(0));
    //         let option = scorer.score(row, s1, 0, s2);
    //         if option.0 > result.0 {
    //             result = (option.0, option.1.into())
    //         }
    //
    //         // Gap row?
    //         let option = self.cache[row - 1] - scorer.gaprow(row, row + 1);
    //         if option > result.0 {
    //             result = (option, Trace::GapRow)
    //         }
    //
    //         self.cache.push(result.0);
    //         tracer.update(row, 0, result.1);
    //         storage.step(row, 0, result.0, result.1);
    //     }
    //     debug_assert!(self.cache.len() == seq1.len());
    //     storage.on_column_end();
    // }
}

impl Aligner for DynProg {
    fn align(
        &mut self,
        seq1: &impl Alignable,
        seq2: &impl Alignable,
        scorer: &mut impl ScoringFunc,
        tracer: &mut impl Tracer,
        storage: &mut impl Storage,
    ) {
        todo!()
        // // The first column is a special case
        // self.solve_first_col(seq1, seq2, scorer, tracer, storage);
        //
        // for col in 1..seq2.len() {
        //     let mut buffer = self.cache[0];
        //
        //     // The first row is a special case
        //     self.solve_first_row(col, seq1, seq2, scorer, tracer, storage);
        //
        //     for row in 1..seq1.len() {
        //         // At each position we consider the following "predict" rectangle
        //         // buffer    | scores[i - 1]
        //         // scores[i] | result
        //         //
        //         // result = max(match, gap left, gap top)
        //
        //         let mut result = (0, Trace::None);
        //
        //         // Match?
        //         let (s1, s2) = (seq1.at(row), seq2.at(col));
        //         let option = scorer.score(row, s1, col, s2);
        //         if buffer + option.0 > result.0 {
        //             result = (buffer + option.0, option.1.into())
        //         }
        //
        //         // Gap column?
        //         let option = self.cache[row] + scorer.gapcol(col, col + 1);
        //         if option > result.0 {
        //             result = (option, Trace::GapCol);
        //         }
        //
        //         // Gap row?
        //         let option = self.cache[row - 1] + scorer.gaprow(row, row + 1);
        //         if option > result.0 {
        //             result = (option, Trace::GapRow);
        //         }
        //
        //         buffer = self.cache[row];
        //         self.cache[row] = result.0;
        //
        //         tracer.update(row, col, result.1);
        //         storage.step(row, col, result.0, result.1);
        //     }
        //     storage.on_column_end();
        // }
    }
}
