use std::ops::Range;

pub use tracemat::TraceMatrix;

use super::AlignmentOps;

mod tracemat;

#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Hash)]
pub enum Trace {
    #[default]
    None,
    GapRow,
    GapCol,
    Match,
    Mismatch,
}

pub struct TracedAlignment {
    pub ops: Vec<AlignmentOps>,
    pub seq1range: Range<usize>,
    pub seq2range: Range<usize>,
}

pub trait Tracer {
    fn reset(&mut self, rows: usize, cols: usize);

    fn gap_row(&mut self, row: usize, col: usize);
    fn gap_col(&mut self, row: usize, col: usize);
    fn matched(&mut self, row: usize, col: usize);
    fn mismatched(&mut self, row: usize, col: usize);

    fn trace(&self, row: usize, col: usize) -> Result<TracedAlignment, ()>;
}

#[cfg(test)]
pub mod test_suite {
    use super::*;

    struct Workload {
        seed: (usize, usize),
        seq1: (usize, usize),
        seq2: (usize, usize),
        ops: Vec<AlignmentOps>,
    }

    fn fill_tracer(tracer: &mut impl Tracer, rows: usize, cols: usize, traces: &[Trace]) {
        debug_assert!(traces.len() == rows * cols);
        tracer.reset(rows, cols);
        for r in 0..rows {
            for c in 0..cols {
                let ind = r * cols + c;
                match traces[ind] {
                    Trace::None => {}
                    Trace::GapRow => tracer.gap_row(r, c),
                    Trace::GapCol => tracer.gap_col(r, c),
                    Trace::Match => tracer.matched(r, c),
                    Trace::Mismatch => tracer.mismatched(r, c),
                }
            }
        }
    }

    fn ensure(tracer: &mut impl Tracer, w: Workload) {
        let trace = tracer.trace(w.seed.0, w.seed.1).unwrap();
        assert_eq!(trace.ops, w.ops);
        assert_eq!(
            trace.seq1range,
            Range {
                start: w.seq1.0,
                end: w.seq1.1
            },
            "Seq 1 ranges mismatch"
        );
        assert_eq!(
            trace.seq2range,
            Range {
                start: w.seq2.0,
                end: w.seq2.1
            },
            "Seq 2 ranges mismatch"
        );
    }

    pub fn run_all(tracer: &mut impl Tracer) {
        outside_range(tracer);
        simple(tracer);
        complex(tracer);
    }

    fn outside_range(tracer: &mut impl Tracer) {
        for [(size_row, size_col), (trace_row, trace_col)] in [
            [(0, 0), (0, 0)],
            [(0, 0), (0, 1)],
            [(11, 12), (11, 12)],
            [(11, 12), (10, 13)],
        ] {
            tracer.reset(size_row, size_col);
            assert!(tracer.trace(trace_row, trace_col).is_err())
        }
    }

    fn simple(tracer: &mut impl Tracer) {
        tracer.reset(4, 4);
        tracer.matched(0, 0);
        tracer.matched(1, 1);
        tracer.gap_row(2, 1);
        tracer.gap_col(2, 2);
        tracer.mismatched(3, 3);

        for (row, col) in [(0, 0), (1, 1), (2, 1), (2, 2), (3, 3)] {
            assert!(tracer.trace(row, col).is_ok());
        }
        for (row, col) in [(0, 1), (4, 4), (3, 2), (1, 2)] {
            assert!(tracer.trace(row, col).is_err());
        }

        ensure(
            tracer,
            Workload {
                seed: (3, 3),
                seq1: (0, 4),
                seq2: (0, 4),
                ops: vec![
                    AlignmentOps::Match(2),
                    AlignmentOps::GapFirst(1),
                    AlignmentOps::GapSecond(1),
                    AlignmentOps::Mismatch(1),
                ],
            },
        )
    }

    fn complex(tracer: &mut impl Tracer) {
        use Trace::*;
        let traces = vec![
            Match, None, Match, None, None, Mismatch, Match, GapCol, None, Match, GapCol, None,
            None, GapRow, Match, None, None, Mismatch, None, Match, None, Mismatch, None, None,
            GapCol, Mismatch, Match, None, Mismatch, None, None, None, Mismatch, Match, None,
            Mismatch, GapCol, None, None, GapRow, GapCol, GapRow, None, GapRow, None, GapCol,
            Match, Match,
        ];
        fill_tracer(tracer, 8, 6, &traces);
        let workload = vec![
            Workload {
                seed: (7, 1),
                seq1: (7, 8),
                seq2: (1, 1),
                ops: vec![AlignmentOps::GapFirst(1)],
            },
            Workload {
                seed: (6, 0),
                seq1: (6, 6),
                seq2: (0, 1),
                ops: vec![AlignmentOps::GapSecond(1)],
            },
            Workload {
                seed: (0, 0),
                seq1: (0, 1),
                seq2: (0, 1),
                ops: vec![AlignmentOps::Match(1)],
            },
            Workload {
                seed: (0, 5),
                seq1: (0, 1),
                seq2: (5, 6),
                ops: vec![AlignmentOps::Mismatch(1)],
            },
            Workload {
                seed: (2, 5),
                seq1: (0, 3),
                seq2: (2, 6),
                ops: vec![
                    AlignmentOps::Match(2),
                    AlignmentOps::GapSecond(1),
                    AlignmentOps::Mismatch(1),
                ],
            },
            Workload {
                seed: (7, 5),
                seq1: (3, 8),
                seq2: (1, 6),
                ops: vec![
                    AlignmentOps::Match(3),
                    AlignmentOps::GapFirst(1),
                    AlignmentOps::GapSecond(1),
                    AlignmentOps::Match(1),
                ],
            },
            Workload {
                seed: (7, 4),
                seq1: (3, 8),
                seq2: (1, 5),
                ops: vec![
                    AlignmentOps::Match(3),
                    AlignmentOps::GapFirst(1),
                    AlignmentOps::Match(1),
                ],
            },
            Workload {
                seed: (6, 5),
                seq1: (1, 7),
                seq2: (0, 6),
                ops: vec![
                    AlignmentOps::Match(1),
                    AlignmentOps::GapSecond(1),
                    AlignmentOps::Match(1),
                    AlignmentOps::Mismatch(3),
                    AlignmentOps::GapFirst(1),
                ],
            },
            Workload {
                seed: (5, 2),
                seq1: (4, 6),
                seq2: (1, 3),
                ops: vec![AlignmentOps::Mismatch(2)],
            },
            Workload {
                seed: (2, 1),
                seq1: (1, 3),
                seq2: (0, 2),
                ops: vec![
                    AlignmentOps::Match(1),
                    AlignmentOps::GapSecond(1),
                    AlignmentOps::GapFirst(1),
                ],
            },
        ];
        for w in workload {
            ensure(tracer, w);
        }
    }
}
