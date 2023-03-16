// use crate::align::pairwise::aligner::Aligner;
// use crate::align::pairwise::storage::Storage;
// use crate::align::pairwise::traceback::Tracer;
// use crate::align::pairwise::{Score, Symbol};
//
// use super::alignable::Alignable;
// use super::{aligner, scoring, storage, traceback, Alignment};
//
// type GapsScore = scoring::gaps::AffineScheme;
// type SymbScore = scoring::symbols::Matrix<Symbol>;
//
// pub struct Engine {
//     algo: aligner::DynProg,
//     storage: storage::AllOptimal,
//     tracer: traceback::TraceMatrix,
//     pub scorer: scoring::Delegate<GapsScore, SymbScore>,
// }
//
// impl Engine {
//     pub fn new(gapscheme: GapsScore, submatrix: SymbScore, minlen: usize, minscore: Score) -> Self {
//         let scorer = scoring::Delegate {
//             gap: gapscheme,
//             symbols: submatrix,
//         };
//         Self {
//             algo: aligner::DynProg::new(0),
//             storage: storage::AllOptimal::new(0, minlen, minscore),
//             tracer: traceback::TraceMatrix::new(),
//             scorer,
//         }
//     }
// }
//
// impl Engine {
//     pub fn align<S1: Alignable, S2: Alignable>(&mut self, seq1: S1, seq2: S2) -> Vec<Alignment> {
//         self.storage.prepare(seq1.len(), seq2.len());
//         self.tracer.reset(seq1.len(), seq2.len());
//
//         self.algo.align(
//             &seq1,
//             &seq2,
//             &mut self.scorer,
//             &mut self.tracer,
//             &mut self.storage,
//         );
//         let results = self.storage.finalize();
//
//         let elements = results.iter().map(|x| x.len()).sum();
//         let mut alignments = Vec::with_capacity(elements);
//
//         for path in results.into_iter().flatten() {
//             let trace = self.tracer.trace(path.end.0, path.end.1).unwrap();
//             debug_assert!(
//                 trace.seq1range.start == path.start.0 && trace.seq1range.end == path.end.0
//             );
//             debug_assert!(
//                 trace.seq2range.start == path.start.1 && trace.seq2range.end == path.end.1
//             );
//
//             alignments.push(Alignment {
//                 score: path.score,
//                 ops: trace.ops,
//                 seq1range: trace.seq1range,
//                 seq2range: trace.seq2range,
//             });
//         }
//         return alignments;
//     }
// }
