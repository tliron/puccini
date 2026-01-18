use {
    problemo::*,
    pyo3::{exceptions::*, prelude::*},
};

//
// IntoPyErr
//

/// Into Python error.
pub trait IntoPyErr {
    /// Into Python error.
    fn into_py(self) -> PyErr;
}

impl IntoPyErr for Problem {
    fn into_py(self) -> PyErr {
        PyException::new_err(self.to_string())
    }
}
