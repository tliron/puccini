use super::super::utils::*;

use {pyo3::prelude::*, read_url::*};

//
// PyContext
//

/// URL context.
#[pyclass(frozen, name = "Context")]
pub struct PyUrlContext {
    /// Inner URL context reference.
    pub inner: UrlContextRef,
}

#[pymethods]
impl PyUrlContext {
    /// Constructor.
    #[new]
    #[pyo3(signature = (with_working_dir=true))]
    pub fn new(with_working_dir: bool) -> PyResult<Self> {
        let inner = UrlContext::new();
        let inner = if with_working_dir { inner.with_base_urls(inner.working_dir_url_vec().into_py()?) } else { inner };
        Ok(Self { inner })
    }
}
