use super::super::utils::*;

use {
    compris::{annotate::*, normal::*},
    floria::*,
    problemo::{common::*, *},
    pyo3::prelude::*,
};

//
// PyStore
//

/// Floria store.
#[pyclass(frozen, name = "Store")]
pub struct PyStore {
    /// Inner store reference.
    pub inner: StoreRef,
}

#[pymethods]
impl PyStore {
    /// Constructor.
    #[new]
    pub fn new() -> Self {
        Self { inner: InMemoryStore::default().as_ref() }
    }

    /// Get vertex template.
    pub fn get_vertex_template<'py>(&self, py: Python<'py>, id: &str) -> PyResult<Bound<'py, PyAny>> {
        let id = ID::parse(EntityKind::VertexTemplate, &id).into_py()?;
        self._get_vertex_template(id).map_err(|problem| problem.into_py()).and_then(|variant| variant.into_py(py))
    }
}

impl PyStore {
    fn _get_vertex_template(&self, id: ID) -> Result<Variant<WithoutAnnotations>, Problem> {
        match self.inner.get_vertex_template(&id)? {
            Some(vertex_template) => Ok(vertex_template.into_expression(true, self.inner.clone())?.into()),
            None => Err("vertex template not found".gloss()),
        }
    }
}
