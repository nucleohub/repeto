pub use delegate::Delegate;

pub mod delegate;
pub mod gaps;
pub mod symbols;

pub trait ScoringScheme: gaps::Scorer + symbols::Scorer {}
