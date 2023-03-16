use std::ops::Range;

use super::super::AlignmentOps;
use super::{Trace, TracedAlignment, Tracer};

struct RunningTrace {
    pub op: Trace,
    pub len: u8,
}

impl RunningTrace {
    pub fn new(trace: Trace) -> Self {
        Self { op: trace, len: 1 }
    }

    pub fn to_alignment_op(self) -> AlignmentOps {
        debug_assert!(self.len > 0);
        match self.op {
            Trace::None => {
                unreachable!("TraceMatrix was corrupted or not calculated correctly.")
            }
            Trace::GapRow => AlignmentOps::GapFirst(self.len),
            Trace::GapCol => AlignmentOps::GapSecond(self.len),
            Trace::Match => AlignmentOps::Match(self.len),
            Trace::Mismatch => AlignmentOps::Mismatch(self.len),
        }
    }
}

pub struct TraceMatrix {
    mat: Vec<Trace>,
    rows: usize,
    cols: usize,
}

impl TraceMatrix {
    pub fn new() -> Self {
        Self {
            mat: Vec::new(),
            rows: 0,
            cols: 0,
        }
    }
}

impl Tracer for TraceMatrix {
    fn reset(&mut self, rows: usize, cols: usize) {
        self.rows = rows + 1;
        self.cols = cols + 1;

        self.mat.clear();
        self.mat.resize(self.rows * self.cols, Trace::None);
    }

    #[inline(always)]
    fn gap_row(&mut self, row: usize, col: usize) {
        self.mat[(row + 1) * self.cols + (col + 1)] = Trace::GapRow;
    }

    #[inline(always)]
    fn gap_col(&mut self, row: usize, col: usize) {
        self.mat[(row + 1) * self.cols + (col + 1)] = Trace::GapCol;
    }

    #[inline(always)]
    fn matched(&mut self, row: usize, col: usize) {
        self.mat[(row + 1) * self.cols + (col + 1)] = Trace::Match;
    }

    #[inline(always)]
    fn mismatched(&mut self, row: usize, col: usize) {
        self.mat[(row + 1) * self.cols + (col + 1)] = Trace::Mismatch;
    }

    fn trace(&self, row: usize, col: usize) -> Result<TracedAlignment, ()> {
        let (seq1end, seq2end) = (row + 1, col + 1);
        if seq1end >= self.rows || seq2end >= self.cols {
            return Err(());
        }

        let (mut row, mut col) = (seq1end, seq2end);
        let seed = self.mat[row * self.cols + col];
        if seed == Trace::None {
            return Err(());
        }

        let mut result = Vec::new();
        let mut trace = RunningTrace::new(seed);
        trace.len = 0;

        loop {
            let op = self.mat[row * self.cols + col];

            if op == Trace::None {
                result.push(trace.to_alignment_op());
                break;
            } else if op == trace.op {
                trace.len += 1;
            } else {
                result.push(trace.to_alignment_op());
                trace = RunningTrace::new(op);
            }

            match op {
                Trace::None => {
                    break;
                }
                Trace::GapRow => {
                    row -= 1;
                }
                Trace::GapCol => {
                    col -= 1;
                }
                Trace::Match | Trace::Mismatch => {
                    row -= 1;
                    col -= 1;
                }
            };
        }
        let mut seq1range = Range {
            start: row,
            end: seq1end,
        };
        let mut seq2range = Range {
            start: col,
            end: seq2end,
        };
        for x in [&mut seq1range, &mut seq2range] {
            if x.start == x.end {
                x.start -= 1;
                x.end -= 1;
            }
        }
        result.reverse();

        return Ok(TracedAlignment {
            ops: result,
            seq1range,
            seq2range,
        });
    }
}

#[cfg(test)]
mod test {
    use super::super::test_suite;
    use super::*;

    #[test]
    fn test() {
        let mut tracer = TraceMatrix::new();
        test_suite::run_all(&mut tracer);
    }
}
