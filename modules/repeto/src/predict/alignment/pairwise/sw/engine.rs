use super::{Alignable, Alignment};
use super::super::scoring::ScoringScheme;
use super::storage::Storage;
use super::traceback::Tracer;

pub struct Engine<S: Storage, T: Tracer, SF: ScoringScheme> {
    // algo: A,
    storage: S,
    tracer: T,
    scheme: SF
}

// impl<A: SWAlgo<S, SF, T>, S: Storage, T: Tracer, SF: ScoringScheme> Engine<A, S, T, SF> {
//     pub fn run<S1: Alignable, S2: Alignable>(&mut self, seq1: S1, seq2: S2) -> Vec<Alignment> {
//         self.cnx.storage.reset(seq1.len(), seq2.len());
//         self.cnx.tracer.reset(seq1.len(), seq2.len());

        // self.algo.alignment(&mut self);
        //
        // self.storage.finalize().into_iter().map(|x| {
        //     let trace = self.tracer.trace(x.row, x.col).unwrap();
        //     debug_assert!(trace.seq1range.end == x.row && trace.seq2range.end == x.col);
        //
        //     Alignment {
        //         score: x.score,
        //         ops: trace.ops,
        //         seq1range: trace.seq1range,
        //         seq2range: trace.seq2range,
        //     }
        // }).collect()
    // }
// }
