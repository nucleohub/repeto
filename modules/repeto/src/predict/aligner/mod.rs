use super::alignable::Alignable;
use super::scoring::ScoringFunc;
use super::storage::Storage;
use super::traceback::Tracer;

mod dynprog;

pub use dynprog::DynProg;

pub trait Aligner {
    fn align(
        &mut self,
        seq1: &impl Alignable,
        seq2: &impl Alignable,
        scorer: &mut impl ScoringFunc,
        tracer: &mut impl Tracer,
        storage: &mut impl Storage,
    );
}
