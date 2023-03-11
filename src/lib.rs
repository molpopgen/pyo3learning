use pyo3::prelude::*;

#[pyfunction]
pub fn add(left: usize, right: usize) -> PyResult<usize> {
    Ok(left + right)
}

#[pymodule]
fn pyo3learning(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(add, m)?)?;
    Ok(())
}
