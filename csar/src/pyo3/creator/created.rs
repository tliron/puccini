use super::super::{super::creator::*, meta::*};

use pyo3::prelude::*;

//
// PyCreatedCsar
//

/// Created CSAR.
#[pyclass(name = "Created", frozen)]
pub struct PyCreatedCsar {
    /// Inner created CSAR.
    pub inner: CreatedCsar,
}

#[pymethods]
impl PyCreatedCsar {
    /// TOSCA meta.
    #[getter]
    pub fn tosca_meta(&self) -> PyToscaMeta {
        self.inner.tosca_meta.clone().into()
    }

    /// Format.
    #[getter]
    pub fn format(&self) -> String {
        self.inner.format.to_string()
    }

    /// Compression level.
    #[getter]
    pub fn compression_level(&self) -> Option<usize> {
        self.inner.compression_level.map(|compression_level| compression_level.into())
    }

    /// Size.
    #[getter]
    pub fn size(&self) -> Option<u64> {
        self.inner.size.clone()
    }
}

impl From<CreatedCsar> for PyCreatedCsar {
    fn from(inner: CreatedCsar) -> Self {
        Self { inner }
    }
}
