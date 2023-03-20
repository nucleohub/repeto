pub use local::FullScan;

use super::storage::Storage;
use super::super::{ConstrainedPos, Constraint};
use super::super::scoring::ScoringScheme;
use super::super::super::Alignable;
use super::traceback::Tracer;

mod local;

// use super::super::storage::Storage;
// use super::super::super::scoring::ScoringScheme;
// use super::super::traceback::Tracer;
//
// pub struct Context<S: Storage, SF: ScoringScheme, T: Tracer> {
//     pub storage: S,
//     pub scoring: SF,
//     pub tracer: T,
// }
