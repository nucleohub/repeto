pub use delegate::Delegate;

pub mod delegate;
pub mod gaps;
pub mod symbols;

pub trait ScoringFunc: gaps::ScoringFunc + symbols::ScoringFunc {}
