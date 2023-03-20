// pub use alloptimal::AllOptimal;

use super::Score;
use super::traceback::Trace;

mod alloptimal;

pub struct AlignmentSeed {
    pub row: usize,
    pub col: usize,
    pub score: Score,
}

pub trait Storage {
    fn reset(&mut self, newrows: usize, newcols: usize);

    fn gap_row(&mut self, row: usize, col: usize, score: Score);
    fn gap_col(&mut self, row: usize, col: usize, score: Score);
    fn equivalent(&mut self, row: usize, col: usize, score: Score);
    fn restart(&mut self, row: usize, col: usize);

    fn finalize(&mut self) -> Vec<AlignmentSeed>;

    fn on_column_end(&mut self);
}
