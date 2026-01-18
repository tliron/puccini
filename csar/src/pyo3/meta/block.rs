use super::super::super::tosca_meta::*;

use {
    compris::pyo3::problemo::*,
    problemo::*,
    pyo3::{prelude::*, types::*},
};

//
// PyToscaMetaBlock
//

/// TOSCA meta block.
///
/// The order of keys is preserved for roundtrips.
#[pyclass(name = "MetaBlock", mapping)]
pub struct PyToscaMetaBlock {
    /// Inner TOSCA meta block.
    pub inner: ToscaMetaBlock,
}

#[pymethods]
impl PyToscaMetaBlock {
    /// Constructor.
    #[new]
    pub fn new() -> Self {
        ToscaMetaBlock::default().into()
    }

    /// Parse item as a list of strings.
    pub fn get_list(&self, keyname: &str) -> PyResult<Vec<String>> {
        Ok(self.inner.get_list(keyname, &mut FailFast).into_py()?.unwrap_or_else(|| Vec::default()))
    }

    /// Length.
    pub fn __len__(&self) -> usize {
        self.inner.len()
    }

    /// Iterate keys.
    pub fn __iter__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyIterator>> {
        let tuple = PyTuple::new(py, self.inner.keynames()).into_py()?;
        tuple.as_any().try_iter()
    }

    /// Get item as string.
    pub fn __getitem__(&self, keyname: &str) -> Option<String> {
        self.inner.get(keyname).cloned()
    }

    /// Set item (list or string).
    pub fn __setitem__<'py>(&mut self, keyname: String, value: Bound<'py, PyAny>) -> PyResult<()> {
        if let Ok(value) = value.extract() {
            self.inner.insert_list(keyname, &value).into_py()
        } else if let Ok(value) = value.extract() {
            self.inner.insert(keyname, value).into_py()
        } else {
            todo!()
        }
    }

    /// Delete item.
    pub fn __delitem__(&mut self, keyname: &str) -> PyResult<()> {
        self.inner.remove(keyname).into_py()
    }

    /// As string.
    pub fn __str__(&self) -> String {
        self.inner.stringify(None)
    }
}

impl From<ToscaMetaBlock> for PyToscaMetaBlock {
    fn from(inner: ToscaMetaBlock) -> Self {
        Self { inner }
    }
}
