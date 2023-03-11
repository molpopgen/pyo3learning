use pyo3::prelude::*;

#[pyfunction]
pub fn add(left: usize, right: usize) -> PyResult<usize> {
    Ok(left + right)
}

#[pyclass]
struct HoldsVec {
    data: Vec<i32>,
}

#[pymethods]
impl HoldsVec {
    #[new]
    fn new(data: Vec<i32>) -> Self {
        Self { data }
    }
}

#[pymodule]
fn pyo3learning(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(add, m)?)?;
    m.add_class::<HoldsVec>()?;
    Ok(())
}
