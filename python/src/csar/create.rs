use pyo3::prelude::*;

/// Create CSAR.
#[pyfunction(name = "create")]
pub fn py_create(url: String) -> PyResult<String> {
    Ok(url)
}
