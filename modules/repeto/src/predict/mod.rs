use std::ops::Range;

pub type Score = i32;
pub type Symbol = u8;

pub mod alignable;
pub mod aligner;
pub mod engine;
pub mod scoring;
pub mod storage;
pub mod traceback;

#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum AlignmentOps {
    GapFirst(u8),
    GapSecond(u8),
    Match(u8),
    Mismatch(u8),
}

pub struct Alignment {
    pub score: Score,
    pub ops: Vec<AlignmentOps>,
    pub seq1range: Range<usize>,
    pub seq2range: Range<usize>,
}
