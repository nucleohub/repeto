use std::ops::Range;

pub use constraint::{ConstrainedPos, Constraint};

pub use super::{Alignable, Score, Symbol};

pub mod sw;
pub mod scoring;
mod constraint;

#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum AlignmentOp {
    GapFirst,
    GapSecond,

    // Equivalence = ambiguous, i.e. match OR mismatch
    Equivalent,
    Match,
    Mismatch,
}

// TODO: bitpack using 16 bit value?
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct AlignmentStep {
    pub op: AlignmentOp,
    pub len: u8,
}


pub struct Alignment {
    pub score: Score,
    pub ops: Vec<AlignmentStep>,
    pub seq1range: Range<usize>,
    pub seq2range: Range<usize>,
}

pub trait Aligner<S1: Alignable, S2: Alignable> {
    // Unconstrained alignment
    fn align(&mut self, seq1: &S1, seq2: &S2) {
        self.align_constrained(seq1, &[], seq2, &[])
    }

    // Constrained alignment
    fn align_constrained(&mut self,
                         seq1: &S1, seq1cons: &[ConstrainedPos],
                         seq2: &S2, seq2cons: &[ConstrainedPos]);
}
