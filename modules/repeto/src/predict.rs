use std::ops::Range;
use biobit_alignment::alignable::Reversed;
use biobit_alignment::pairwise::{backend, scoring, AlignmentOp};
use biobit_alignment::pairwise::local::{MultiAligner, MultiAlignerConfig};

use super::repeats::inv;

pub fn run(seq: &[u8], min_score: i64, min_matches_run: usize) -> (Vec<inv::Repeat>, Vec<i64>) {
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
    let mut scores = Vec::with_capacity(alignments.len());
    let alignments = alignments.into_iter().filter_map(|x| {
        let mut segments = Vec::with_capacity(x.steps.len());

        let mut max_matches_run = 0;
        for step in x.coalesced_steps() {
            match step.op {
                AlignmentOp::Equivalent => { unreachable!() }
                AlignmentOp::Match => {
                    max_matches_run = max_matches_run.max(step.len);

                    let left = Range {
                        start: (step.start.seq2) as isize,
                        end: (step.start.seq2 + step.len) as isize
                    };
                    let right = Range {
                        start: (seq.len() - step.start.seq1 - step.len) as isize,
                        end: (seq.len() - step.start.seq1) as isize
                    };
                    segments.push(inv::Segment::new(left, right));
                }
                AlignmentOp::Mismatch | AlignmentOp::GapSecond | AlignmentOp::GapFirst => {}
            }
        }
        if max_matches_run >= min_matches_run {
            scores.push(x.score);
            Some(inv::Repeat::new(segments))
        } else {
            None
        }
    }).collect();
    (alignments, scores)
}
