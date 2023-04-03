use std::fmt::{Display, Formatter};

use pyo3::prelude::*;

use repeto;
use repeto::InvertedRepeat;

#[pyclass(get_all, set_all)]
#[derive(Clone, Debug)]
pub struct PyRange {
    pub start: usize,
    pub end: usize,
}

#[pymethods]
impl PyRange {
    fn __repr__(&self) -> String {
        format!("[{}, {})", self.start, self.end)
    }
}

#[pyclass(get_all, set_all)]
#[derive(Clone, Debug)]
pub struct PyRepeatSegment {
    pub left: Py<PyRange>,
    pub right: Py<PyRange>,
}


#[pymethods]
impl PyRepeatSegment {
    fn __repr__(&self) -> String {
        Python::with_gil(|py| {
            format!(
                "RepeatSegment {{ {} <-> {} }}",
                self.left.borrow(py).__repr__(),
                self.right.borrow(py).__repr__()
            )
        })
    }
}

#[pyclass(get_all, set_all)]
#[derive(Clone, Debug)]
pub struct PyInvertedRepeat {
    pub segments: Vec<Py<PyRepeatSegment>>,
}

impl PyInvertedRepeat {
    pub fn unbox(self, py: Python) -> repeto::InvertedRepeat {
        let segments: Vec<_> = self.segments.iter().map(|x| {
            let x = x.borrow(py);
            let (left, right) = (x.left.borrow(py), x.right.borrow(py));
            repeto::Segment::new(left.start..left.end, right.start..right.end)
        }).collect();
        repeto::InvertedRepeat::new(segments)
    }

    pub fn boxr(ir: &InvertedRepeat, py: Python) -> PyResult<Self> {
        let segments: PyResult<Vec<Py<PyRepeatSegment>>> = ir.segments().iter().map(|s| {
            Py::new(py, PyRepeatSegment {
                left: Py::new(py, PyRange {
                    start: s.left().start,
                    end: s.left().end,
                })?,
                right: Py::new(py, PyRange {
                    start: s.right().start,
                    end: s.right().end,
                })?,
            })
        }).collect();
        Ok(PyInvertedRepeat { segments: segments? })
    }
}

#[pyfunction]
pub fn predict(seq: &[u8], minscore: i64, min_matches_run: usize) -> PyResult<Vec<PyInvertedRepeat>> {
    let results = repeto::predict(seq, minscore, min_matches_run);

    // Convert to Py-wrappers
    Python::with_gil(|py| -> PyResult<Vec<PyInvertedRepeat>>{
        results.into_iter().map(|ir| PyInvertedRepeat::boxr(&ir, py)).collect()
    })
}


#[pyfunction]
pub fn optimize(ir: Vec<PyInvertedRepeat>, scores: Vec<i32>) -> PyResult<(Vec<PyInvertedRepeat>, i32)> {
    let ir = Python::with_gil(|py| -> Vec<repeto::InvertedRepeat> {
        ir.into_iter().map(|x| { x.unbox(py) }).collect()
    });

    let (result, total_score) = repeto::optimize(&ir, &scores);

    let ir = Python::with_gil(|py| -> PyResult<Vec<PyInvertedRepeat>> {
        result.into_iter().map(
            |x| PyInvertedRepeat::boxr(x, py)
        ).collect()
    })?;
    return Ok((ir, total_score));
}


#[pymodule]
#[pyo3(name = "repeto")]
fn py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyRange>()?;
    m.add_class::<PyRepeatSegment>()?;
    m.add_class::<PyInvertedRepeat>()?;
    m.add_function(wrap_pyfunction!(predict, m)?)?;
    m.add_function(wrap_pyfunction!(optimize, m)?)?;
    Ok(())
}