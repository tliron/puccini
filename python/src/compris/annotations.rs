use super::location::*;

use {
    compris::annotate::*,
    depiction::*,
    pyo3::{prelude::*, types::*},
};

//
// PyAnnotations
//

/// Annotations.
#[pyclass(name = "Annotations", frozen)]
pub struct PyAnnotations {
    /// Inner annotations.
    pub inner: Annotations,
}

#[pymethods]
impl PyAnnotations {
    /// Source.
    #[getter]
    pub fn source(&self) -> Option<String> {
        self.inner.source.clone().map(|source| source.into())
    }

    /// Start.
    #[getter]
    pub fn start<'py>(&self, py: Python<'py>) -> PyResult<Option<Bound<'py, PyLocation>>> {
        Ok(match &self.inner.span {
            Some(span) => Some(Bound::new(py, PyLocation::from(span.start.clone()))?),
            None => None,
        })
    }

    /// End.
    #[getter]
    pub fn end<'py>(&self, py: Python<'py>) -> PyResult<Option<Bound<'py, PyLocation>>> {
        Ok(match &self.inner.span {
            Some(span) => match &span.end {
                Some(end) => Some(Bound::new(py, PyLocation::from(end.clone()))?),
                None => None,
            },
            None => None,
        })
    }

    /// Path.
    #[getter]
    pub fn path(&self) -> Option<String> {
        self.inner.path.as_ref().map(|path| path.to_string())
    }

    /// Label.
    #[getter]
    pub fn label<'py>(&self, py: Python<'py>) -> Option<Bound<'py, PyAny>> {
        self.inner.label.as_ref().map(|label| label_to_py(label, py))
    }

    /// As string.
    pub fn __str__(&self) -> String {
        self.to_depiction(&DEFAULT_DEPICTION_CONTEXT)
    }
}

impl PyAnnotations {
    /// To depiction.
    pub fn to_depiction(&self, context: &DepictionContext) -> String {
        self.inner.to_depiction(context).expect("to_depiction")
    }
}

impl From<Annotations> for PyAnnotations {
    fn from(inner: Annotations) -> Self {
        Self { inner }
    }
}

// Utils

fn label_to_py<'py>(label: &Label, py: Python<'py>) -> Bound<'py, PyAny> {
    match label {
        Label::Integer(integer) => PyInt::new(py, *integer).into_any(),
        Label::String(string) => PyString::new(py, &string).into_any(),
    }
}
