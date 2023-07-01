use itertools::Itertools;
use pyo3::prelude::*;

use repeats::{InvertedRepeat, Range, RepeatSegment};
use repeto;

mod repeats;

#[pyfunction]
pub fn predict(seq: &[u8], min_score: i64, min_matches_run: usize) -> PyResult<(Vec<InvertedRepeat>, Vec<i64>)> {
    let (ir, scores) = repeto::predict::run(seq, min_score, min_matches_run);

    // Convert to Py-wrappers
    let ir = Python::with_gil(|py| -> PyResult<Vec<InvertedRepeat>>{
        ir.into_iter().map(|ir| InvertedRepeat::from_rs(&ir, py)).collect()
    })?;

    Ok((ir, scores))
}


#[pyfunction]
pub fn optimize(ir: Vec<Py<InvertedRepeat>>, scores: Vec<i64>) -> PyResult<(Vec<Py<InvertedRepeat>>, i64)> {
    // Transform to an optimized Rust representation
    let rs_ir = Python::with_gil(|py| -> Vec<repeto::repeats::inv::Repeat<isize>> {
        ir.iter().map(|x| { x.borrow(py).to_rs(py) }).collect()
    });

    // Run the solution
    let (solution, total_score) = repeto::optimize::run(&rs_ir, &scores);

    // Shallow copy solution repeats
    let ir = Python::with_gil(|py| {
        solution.into_iter().map(|x| ir[x].clone_ref(py)).collect_vec()
    });

    // Return the result
    return Ok((ir, total_score));
}


#[pymodule]
#[pyo3(name = "repeto")]
fn py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Range>()?;
    m.add_class::<RepeatSegment>()?;
    m.add_class::<InvertedRepeat>()?;
    m.add_function(wrap_pyfunction!(predict, m)?)?;
    m.add_function(wrap_pyfunction!(optimize, m)?)?;
    Ok(())
}
