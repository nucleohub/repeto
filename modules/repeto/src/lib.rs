use predict::alignment::alignable::Reversed;
use predict::alignment::pairwise::{backend, scoring};
pub use repeats::{InvertedRepeat, Segment};

use crate::predict::alignment::pairwise::AlignmentOp;
use crate::predict::alignment::pairwise::local::{MultiAligner, MultiAlignerConfig};

// #[allow(non_snake_case)]
mod repeats;
mod predict;
mod optimize;

pub fn predict(seq: &[u8], min_score: i64, min_matches_run: usize) -> Vec<InvertedRepeat> {
    let mut aligner: backend::sw::Engine<_, _, _> = backend::sw::Engine::new(
        backend::sw::storage::AllOptimal::new(),
        backend::sw::traceback::TraceMatrix::new(),
        scoring::compose(
            scoring::symbols::Complementarity { complementary: 1, otherwise: -2 },
            scoring::gaps::Affine { open: -5, extend: -1 },
        ),
        // scoring::default()
    );
    aligner.set_min_score_thr(min_score);
    // sequence orientation:
    // ------>
    // ^    /
    // |   /
    // |  /
    // | /
    // |/
    let mut alignments = Vec::new();
    aligner.uptriangle(&Reversed::new(seq), &seq, 1, &mut alignments);

    // Convert to segments & inverted repeats
    alignments.into_iter().filter_map(|x| {
        let mut segments = Vec::with_capacity(x.steps.len());

        let mut max_matches_run = 0;
        for step in x.coalesced_steps() {
            match step.op {
                AlignmentOp::Equivalent => { unreachable!() }
                AlignmentOp::Match => {
                    max_matches_run = max_matches_run.max(step.len);
                    segments.push(Segment::new(
                        (step.start.seq2)..(step.start.seq2 + step.len),
                        (seq.len() - step.start.seq1 - step.len)..(seq.len() - step.start.seq1),
                    ))
                }
                AlignmentOp::Mismatch | AlignmentOp::GapSecond | AlignmentOp::GapFirst => {}
            }
        }
        if max_matches_run >= min_matches_run { Some(InvertedRepeat::new(segments)) } else { None }
    }).collect()
}

pub fn optimize<'a>(ir: &'a [InvertedRepeat], scores: &[optimize::Score]) -> (Vec<&'a InvertedRepeat>, optimize::Score) {
    optimize::run(ir, scores)
}
