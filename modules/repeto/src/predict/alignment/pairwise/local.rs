use std::ops::Range;
pub use super::{Alignable, Score, Symbol};
pub use super::alignment::{AlignmentOp, AlignmentStep};
use super::alignment::utils;
pub use super::constraint::{ConstrainedPos, Constraint};
use super::scoring::ScoringScheme;

pub struct AlignmentOffset {
    pub seq1: usize,
    pub seq2: usize,
}

pub struct Alignment {
    pub score: Score,
    pub steps: Vec<AlignmentStep>,
    pub seq1: Range<usize>,
    pub seq2: Range<usize>,
}

impl Alignment {
    pub fn len(&self) -> usize {
        self.steps.iter().map(|x| x.len as usize).sum()
    }

    pub fn rle(&self) -> String {
        utils::rle(&self.steps, self.len())
    }

    pub fn prettify(&self, seq1: &str, seq2: &str) -> String {
        let seq1 = &seq1[self.seq1.start..self.seq1.end];
        let seq2 = &seq2[self.seq2.start..self.seq2.end];
        let total: usize = self.len();
        utils::prettify(seq1, seq2, &self.steps, total)
    }
}

pub trait Aligner<S1: Alignable, S2: Alignable, SF: ScoringScheme> {
    fn reconfigure(&mut self, scoring: SF);
    fn align(&mut self, seq1: &S1, seq2: &S2) -> Result<Alignment, ()>;
}

pub trait MultiAligner<S1: Alignable, S2: Alignable> {
    fn align(&mut self, seq1: &S1, seq2: &S2, saveto: &mut Vec<Alignment>);
}


#[cfg(test)]
pub mod test_suite {
    use super::*;
    use super::super::scoring;

    type TestAligner<'a> = dyn Aligner<
        &'a [u8], &'a [u8],
        scoring::Delegate<scoring::symbols::MatchMismatch, scoring::gaps::Affine>
    >;

    struct Workload<'a> {
        seq1: (&'a [u8], usize),
        seq2: (&'a [u8], usize),
        score: Score,
        rle: &'a str,
    }

    pub mod best {
        use super::*;

        fn ensure<'a>(aligner: &mut TestAligner<'a>, mut w: Workload<'a>) {
            let gapfirst = AlignmentOp::symbol(&AlignmentOp::GapFirst);
            let gapsecond = AlignmentOp::symbol(&AlignmentOp::GapSecond);
            let invrle = w.rle.chars().map(|x|
                if x == gapfirst {
                    gapsecond
                } else if x == gapsecond {
                    gapfirst
                } else {
                    x
                }
            ).collect::<String>();

            for (seq1, seq2, rle) in [
                (w.seq1, w.seq2, w.rle),
                (w.seq2, w.seq1, &invrle)
            ] {
                let result = aligner.align(&seq1.0, &seq2.0).expect(
                    &*format!("Aligner failed: {:?} & {:?}", seq1.0, seq2.0)
                );
                assert_eq!(result.seq1.start, seq1.1);
                assert_eq!(result.seq2.start, seq2.1);
                assert_eq!(result.score, w.score);
                assert_eq!(result.rle(), rle);
            }
        }

        fn test_empty(aligner: &mut TestAligner) {
            let workload: Vec<(&[u8], &[u8])> = vec![
                (b"ACGT", b""), (b"", b"ACGT"), (b"", b""),
                (b"ACGT", b"----"), (b"_", b"A"),
            ];

            for (seq1, seq2) in workload {
                let result = aligner.align(&seq1, &seq2);
                assert!(result.is_err());
            }
        }

        fn test_no_gaps(aligner: &mut TestAligner) {
            let workload = vec![
                Workload {
                    seq1: (b"AAGAA", 1),
                    seq2: (b"AGA", 0),
                    score: 3,
                    rle: "3=",
                },
                Workload {
                    seq1: (b"AGTCCCGTGTCCCAGGGG", 0),
                    seq2: (b"AGTC", 0),
                    score: 4,
                    rle: "4=",
                },
                Workload {
                    seq1: (b"CGCGCGCGTTT", 6),
                    seq2: (b"CGTTT", 0),
                    score: 5,
                    rle: "5=",
                },
                Workload {
                    seq1: (b"AAAGGGAGGGTTTA", 3),
                    seq2: (b"GGGGGGG", 0),
                    score: 4,
                    rle: "3=1X3=",
                },
                Workload {
                    seq1: (b"AAAA", 0),
                    seq2: (b"AAAA", 0),
                    score: 4,
                    rle: "4=",
                },
                Workload {
                    seq1: (b"NNNN==*===*===*==", 7),
                    seq2: (b"++++=============+++", 4),
                    score: 4,
                    rle: "3=1X3=",
                },
                Workload {
                    seq1: (b"NNNN===*===*===*===*===", 4),
                    seq2: (b"===================", 0),
                    score: 7,
                    rle: "3=1X3=1X3=1X3=1X3=",
                },
                Workload {
                    seq1: (b"AGAAAAAAAGGAAAAAAAGGGGG", 1),
                    seq2: (b"G", 0),
                    score: 1,
                    rle: "1=",
                },
            ];

            for mut w in workload {
                ensure(aligner, w);
            }
        }

        fn test_affine_gaps(aligner: &mut TestAligner) {
            let workload = vec![
                Workload {
                    seq1: (b"AAAAAAAAAAAAAAAA*********AAAAAAAAAAAAAAAA", 0),
                    seq2: (b"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA", 0),
                    score: 19,
                    rle: "16=9v16=",
                },
                Workload {
                    seq1: (b"ACGTACGTACGT****_________", 0),
                    seq2: (b"****ACGTACGTACGT_________ACGT*****", 4),
                    score: 13,
                    rle: "12=4v9=",
                },
            ];

            for mut w in workload {
                ensure(aligner, w);
            }
        }

        fn test_free_gap_open(aligner: &mut TestAligner) {
            let workload = vec![
                Workload {
                    seq1: (b"A***AAAAAAAA***AAAAAAAA***A", 4),
                    seq2: (b"AAAAAAAAAAAAAAAA", 0),
                    score: 13,
                    rle: "8=3v8=",
                },
                Workload {
                    seq1: (b"AAAAAAA**AAAAA*****", 0),
                    seq2: (b"___AAAAAAAAAAA", 3),
                    score: 9,
                    rle: "7=2v4=",
                },
            ];

            for mut w in workload {
                ensure(aligner, w);
            }
        }

        pub fn test_all(aligner: &mut TestAligner) {
            aligner.reconfigure(scoring::compose(
                scoring::symbols::MatchMismatch { samesc: 1, diffsc: -2 },
                scoring::gaps::Affine { open: -5, extend: -1 },
            ));
            test_empty(aligner);
            test_no_gaps(aligner);
            test_affine_gaps(aligner);

            aligner.reconfigure(scoring::compose(
                scoring::symbols::MatchMismatch { samesc: 1, diffsc: -2 },
                scoring::gaps::Affine { open: -1, extend: -1 },
            ));
            test_free_gap_open(aligner);
        }
    }
}
