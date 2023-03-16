mod alloptimal;

use super::traceback::Trace;
use super::Score;
pub use alloptimal::AllOptimal;

pub trait Storage {
    type Output;

    fn step(&mut self, row: usize, col: usize, newscore: Score, op: Trace);
    fn on_column_end(&mut self);
    fn prepare(&mut self, newrows: usize, newcols: usize);
    fn finalize(&mut self) -> Self::Output;
}
