use super::super::{compris::*, problemo::*};

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
#[pyclass(name = "Store", frozen)]
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

    /// Get plugin.
    pub fn get_plugin<'py>(&self, py: Python<'py>, id: &str) -> PyResult<Bound<'py, PyAny>> {
        let id = ID::parse(EntityKind::Plugin, &id).into_py()?;
        self._get_plugin(&id).map_err(|problem| problem.into_py()).and_then(|variant| variant.into_py(py))
    }

    /// Get class.
    pub fn get_class<'py>(&self, py: Python<'py>, id: &str) -> PyResult<Bound<'py, PyAny>> {
        let id = ID::parse(EntityKind::Class, &id).into_py()?;
        self._get_class(&id).map_err(|problem| problem.into_py()).and_then(|variant| variant.into_py(py))
    }

    /// Get vertex template.
    #[pyo3(signature = (id, embed=false))]
    pub fn get_vertex_template<'py>(&self, py: Python<'py>, id: &str, embed: bool) -> PyResult<Bound<'py, PyAny>> {
        let id = ID::parse(EntityKind::VertexTemplate, &id).into_py()?;
        self._get_vertex_template(&id, embed)
            .map_err(|problem| problem.into_py())
            .and_then(|variant| variant.into_py(py))
    }

    /// Get edge template.
    #[pyo3(signature = (id, embed=false))]
    pub fn get_edge_template<'py>(&self, py: Python<'py>, id: &str, embed: bool) -> PyResult<Bound<'py, PyAny>> {
        let id = ID::parse(EntityKind::EdgeTemplate, &id).into_py()?;
        self._get_edge_template(&id, embed).map_err(|problem| problem.into_py()).and_then(|variant| variant.into_py(py))
    }

    /// Get vertex.
    #[pyo3(signature = (id, embed=false))]
    pub fn get_vertex<'py>(&self, py: Python<'py>, id: &str, embed: bool) -> PyResult<Bound<'py, PyAny>> {
        let id = ID::parse(EntityKind::Vertex, &id).into_py()?;
        self._get_vertex(&id, embed).map_err(|problem| problem.into_py()).and_then(|variant| variant.into_py(py))
    }

    /// Get edge.
    #[pyo3(signature = (id, embed=false))]
    pub fn get_edge<'py>(&self, py: Python<'py>, id: &str, embed: bool) -> PyResult<Bound<'py, PyAny>> {
        let id = ID::parse(EntityKind::Edge, &id).into_py()?;
        self._get_edge(&id, embed).map_err(|problem| problem.into_py()).and_then(|variant| variant.into_py(py))
    }

    /// Instantiate service template.
    pub fn instantiate_vertex_template(&self, id: &str) -> PyResult<String> {
        // use floria::plugins::*;
        // let environment = PluginEnvironment::new(false)?;
        // let mut context = PluginContext::new(environment, self.inner, url_context);

        // if let Some(plugin) = context.store.get_plugin_by_url(&tosca_2_0::PLUGIN_URL)? {
        //     context.add_dispatch_plugin(
        //         plugin.id,
        //         include_bytes!(concat!(env!("OUT_DIR"), "/puccini_plugin_tosca_2_0.wasm")),
        //         false,
        //     )?;
        // }

        // let mut floria_instance = floria_service_template.instantiate(
        //     &directory,
        //     None,
        //     self.instantiation_payload(inputs).as_ref(),
        //     &mut context,
        //     problems,
        // )?;

        // TODO
        Ok(id.into())
    }
}

impl PyStore {
    fn _get_plugin(&self, id: &ID) -> Result<Variant<WithoutAnnotations>, Problem> {
        match self.inner.get_plugin(id)? {
            Some(plugin) => Ok(Into::<Expression>::into(plugin).into()),
            None => Err("plugin not found".gloss()),
        }
    }

    fn _get_class(&self, id: &ID) -> Result<Variant<WithoutAnnotations>, Problem> {
        match self.inner.get_class(id)? {
            Some(class) => Ok(Into::<Expression>::into(class).into()),
            None => Err("class not found".gloss()),
        }
    }

    fn _get_vertex_template(&self, id: &ID, embed: bool) -> Result<Variant<WithoutAnnotations>, Problem> {
        match self.inner.get_vertex_template(id)? {
            Some(vertex_template) => Ok(vertex_template.into_expression(embed, self.inner.clone())?.into()),
            None => Err("vertex template not found".gloss()),
        }
    }

    fn _get_edge_template(&self, id: &ID, embed: bool) -> Result<Variant<WithoutAnnotations>, Problem> {
        match self.inner.get_edge_template(id)? {
            Some(edge_template) => Ok(edge_template.into_expression(embed, self.inner.clone())?.into()),
            None => Err("edge template not found".gloss()),
        }
    }

    fn _get_vertex(&self, id: &ID, embed: bool) -> Result<Variant<WithoutAnnotations>, Problem> {
        match self.inner.get_vertex(id)? {
            Some(vertex) => Ok(vertex.into_expression(embed, self.inner.clone())?.into()),
            None => Err("vertex not found".gloss()),
        }
    }

    fn _get_edge(&self, id: &ID, embed: bool) -> Result<Variant<WithoutAnnotations>, Problem> {
        match self.inner.get_edge(id)? {
            Some(edge) => Ok(edge.into_expression(embed, self.inner.clone())?.into()),
            None => Err("edge not found".gloss()),
        }
    }
}
