use std::fmt::{Debug, Formatter};
use std::iter::zip;

use itertools::{chain, Itertools};
use pyo3::{PyTraverseError, PyVisit};
use pyo3::prelude::*;
use pyo3::pyclass::CompareOp;
use pyo3::types::PyTuple;

use repeto::repeats;

#[pyclass(get_all, module = "repeto")]
#[derive(Clone, PartialEq, Eq)]
pub struct Range {
    start: isize,
    end: isize,
}

#[pymethods]
impl Range {
    #[new]
    pub fn new(start: isize, end: isize) -> Self {
        assert!(start < end, "Sequence range start must be < end: {:?}", start..end);
        Self { start: start.into(), end: end.into() }
    }

    pub fn shift(&mut self, shift: isize) {
        self.start += shift;
        self.end += shift;
    }

    pub fn __repr__(&self) -> String { format!("{}-{}", self.start, self.end) }

    pub fn __len__(&self) -> usize { (self.end - self.start) as usize }

    pub fn __richcmp__(&self, other: &Self, op: CompareOp, py: Python<'_>) -> PyObject {
        match op {
            CompareOp::Eq => (self == other).into_py(py),
            CompareOp::Ne => (self != other).into_py(py),
            _ => py.NotImplemented(),
        }
    }

    pub fn __getnewargs__(&self) -> PyResult<(isize, isize)> {
        Ok((self.start, self.end))
    }

    #[classattr]
    const __hash__: Option<Py<PyAny>> = None;
}

impl Debug for Range {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.__repr__())
    }
}

#[pyclass(get_all, module = "repeto")]
#[derive(Clone)]
pub struct RepeatSegment {
    left: Py<Range>,
    right: Py<Range>,
}

#[pymethods]
impl RepeatSegment {
    #[new]
    pub fn new(py: Python, left: Py<Range>, right: Py<Range>) -> Self {
        {
            let (l, r) = (left.borrow(py), right.borrow(py));

            assert_eq!(l.__len__(), r.__len__(), "Repeat segments' length must be equal");
            assert!(
                l.start < l.end && l.end <= r.start && r.start < r.end,
                "Repeat segments must not overlap: {:?} vs {:?}", left, right
            );
        }

        Self { left, right }
    }

    pub fn brange(&self, py: Python) -> Range {
        Range {
            start: self.left.borrow(py).start,
            end: self.right.borrow(py).end,
        }
    }

    pub fn shift(&mut self, py: Python, shift: isize) {
        self.left.borrow_mut(py).shift(shift);
        self.right.borrow_mut(py).shift(shift);
    }

    pub fn __repr__(&self, py: Python) -> String {
        format!(
            "RepeatSegment {{ {} <=> {} }}",
            self.left.borrow(py).__repr__(),
            self.right.borrow(py).__repr__()
        )
    }

    pub fn __len__(&self, py: Python) -> usize {
        self.left.borrow(py).__len__()
    }

    pub fn __richcmp__(&self, other: &Self, op: CompareOp, py: Python<'_>) -> PyObject {
        match op {
            CompareOp::Eq => (
                (*self.left.borrow(py) == *other.left.borrow(py)) &&
                    (*self.right.borrow(py) == *other.right.borrow(py))
            ).into_py(py),
            CompareOp::Ne => (
                (*self.left.borrow(py) != *other.left.borrow(py)) ||
                    (*self.right.borrow(py) != *other.right.borrow(py))
            ).into_py(py),
            _ => py.NotImplemented(),
        }
    }

    pub fn __getnewargs__(&self) -> PyResult<(&Py<Range>, &Py<Range>)> {
        Ok((&self.left, &self.right))
    }

    pub fn __traverse__(&self, visit: PyVisit<'_>) -> Result<(), PyTraverseError> {
        visit.call(&self.left)?;
        visit.call(&self.right)?;
        Ok(())
    }

    pub fn __clear__(&mut self) {}

    #[classattr]
    const __hash__: Option<Py<PyAny>> = None;
}

impl Debug for RepeatSegment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Python::with_gil(|py| self.__repr__(py)))
    }
}

#[pyclass(get_all, module = "repeto")]
#[derive(Clone)]
pub struct InvertedRepeat {
    pub segments: Vec<Py<RepeatSegment>>,
}

#[pymethods]
impl InvertedRepeat {
    #[new]
    pub fn new(segments: Vec<Py<RepeatSegment>>, py: Python) -> Self {
        assert!(!segments.is_empty(), "Inverted repeat must have at least one segment");

        // Segments shouldn't overlap
        for (prev, nxt) in segments.iter().tuple_windows() {
            let (p, n) = (prev.borrow(py), nxt.borrow(py));
            assert!(
                p.left.borrow(py).start < n.left.borrow(py).start,
                "Inverted repeat segments must be ordered: {} & {}",
                p.__repr__(py), n.__repr__(py)
            );
            assert!(
                (p.left.borrow(py).end <= n.left.borrow(py).start) &&
                    (p.right.borrow(py).start >= n.right.borrow(py).end),
                "Inverted repeat segments shouldn't overlap: {} & {}",
                p.__repr__(py), n.__repr__(py)
            );
        }

        Self { segments }
    }

    pub fn brange(&self, py: Python) -> Range { self.segments[0].borrow(py).brange(py) }

    pub fn shift(&mut self, py: Python, shift: isize) {
        for s in &mut self.segments {
            s.borrow_mut(py).shift(py, shift);
        }
    }

    pub fn seqranges(&self, py: Python) -> Vec<Py<Range>> {
        chain(
            self.segments.iter().map(|x| x.borrow(py).left.clone_ref(py)),
            self.segments.iter().rev().map(|x| x.borrow(py).right.clone_ref(py)),
        ).collect()
    }

    #[pyo3(
        signature = (contig, *args, name = ".", score = 0, strand = ".", color = "0,0,0"),
        text_signature = None
    )]
    pub fn to_bed12(&self, py: Python, contig: &str,
                    args: &PyTuple, name: &str, score: u16, strand: &str, color: &str) -> String {
        assert_eq!(args.len(), 0, "to_bed12 doesn't support positional arguments except 'contig'.");
        assert!(score <= 1000, "Score must be from 0 to 1000");

        let range = self.brange(py);
        let (block_sizes, block_starts): (Vec<usize>, Vec<isize>) = self.seqranges(py)
            .into_iter()
            .map(|x| (x.borrow(py).__len__(), x.borrow(py).start - range.start))
            .unzip();
        let block_sizes = block_sizes.into_iter().join(",");
        let block_starts = block_starts.into_iter().join(",");

        format!(
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            contig, range.start, range.end, name, score, strand, range.start, range.end, color,
            self.segments.len() * 2, block_sizes, block_starts
        )
    }

    pub fn __len__(&self, py: Python) -> usize {
        self.segments.iter().map(|x| x.borrow(py).__len__(py)).sum()
    }

    pub fn __richcmp__(&self, other: &Self, op: CompareOp, py: Python<'_>) -> PyResult<PyObject> {
        Ok(match op {
            CompareOp::Eq =>
                if self.segments.len() == other.segments.len() {
                    let mut alleq = true;
                    for (a, b) in zip(&self.segments, &other.segments) {
                        let (a, b) = (&*a.borrow(py), &*b.borrow(py));

                        if !a.__richcmp__(b, CompareOp::Eq, py).is_true(py)? {
                            alleq = false;
                            break;
                        }
                    }

                    alleq
                } else {
                    false
                }.into_py(py),
            CompareOp::Ne => (
                !self.__richcmp__(other, CompareOp::Eq, py)?.is_true(py)?
            ).into_py(py),
            _ => py.NotImplemented(),
        })
    }

    pub fn __getnewargs__<'py>(&'py self, py: Python<'py>) -> PyResult<&'py PyTuple> {
        Ok(PyTuple::new(py, &[PyTuple::new(py, &self.segments)]))
    }

    pub fn __traverse__(&self, visit: PyVisit<'_>) -> Result<(), PyTraverseError> {
        for s in &self.segments {
            visit.call(s)?;
        }
        Ok(())
    }

    pub fn __clear__(&mut self) {}

    #[classattr]
    const __hash__: Option<Py<PyAny>> = None;
}


impl InvertedRepeat {
    pub fn to_rs(&self, py: Python) -> repeats::inv::Repeat {
        let segments: Vec<_> = self.segments.iter().map(|x| {
            let x = x.borrow(py);
            let (left, right) = (x.left.borrow(py), x.right.borrow(py));
            repeats::inv::Segment::new(left.start..left.end, right.start..right.end)
        }).collect();
        repeats::inv::Repeat::new(segments)
    }

    pub fn from_rs(ir: &repeats::inv::Repeat, py: Python) -> PyResult<Self> {
        let segments: PyResult<Vec<Py<RepeatSegment>>> = ir.segments().iter().map(|s| {
            Py::new(py, RepeatSegment {
                left: Py::new(py, Range {
                    start: s.left().start,
                    end: s.left().end,
                })?,
                right: Py::new(py, Range {
                    start: s.right().start,
                    end: s.right().end,
                })?,
            })
        }).collect();
        Ok(InvertedRepeat { segments: segments? })
    }
}