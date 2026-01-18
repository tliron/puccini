use {compris::annotate::*, depiction::*, pyo3::prelude::*};

//
// PyLocation
//

/// Location.
#[pyclass(name = "Location", frozen)]
pub struct PyLocation {
    /// Inner location.
    pub inner: Location,
}

#[pymethods]
impl PyLocation {
    /// Index.
    #[getter]
    pub fn index(&self) -> Option<usize> {
        self.inner.index
    }

    /// Row.
    #[getter]
    pub fn row(&self) -> Option<usize> {
        self.inner.row
    }

    /// Column.
    #[getter]
    pub fn column(&self) -> Option<usize> {
        self.inner.column
    }

    /// As string.
    pub fn __str__(&self) -> String {
        self.to_depiction(&DEFAULT_DEPICTION_CONTEXT)
    }
}

impl PyLocation {
    /// To depiction.
    pub fn to_depiction(&self, context: &DepictionContext) -> String {
        self.inner.to_depiction(context).expect("to_depiction")
    }
}

impl From<Location> for PyLocation {
    fn from(inner: Location) -> Self {
        Self { inner }
    }
}
