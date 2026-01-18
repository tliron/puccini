use super::super::{dialect::tosca_2_0, grammar::*};

use {
    ::pyo3::prelude::*,
    compris::{annotate::*, pyo3::problemo::*},
    floria::{pyo3::*, *},
    kutil::pyo3::*,
    problemo::{common::*, *},
    read_url::{pyo3::*, *},
};

/// Compile TOSCA service template.
#[pyfunction(name = "compile_service_template")]
#[pyo3(signature = (url, store, directory=None, url_context=None))]
pub fn py_compile_service_template<'py>(
    url: String,
    store: &Bound<'py, PyAny>,
    directory: Option<String>,
    url_context: Option<&Bound<'py, PyAny>>,
    py: Python<'py>,
) -> PyResult<Bound<'py, PyID>> {
    let store = match store.cast::<PyStore>() {
        Ok(store) => store.get().inner.clone(),
        Err(_) => clone_capsule_attr(store, STORE_CAPSULE_NAME)?,
    };

    let directory = match directory {
        Some(directory) => directory.parse::<Directory>().into_py()?,
        None => Default::default(),
    };

    let url_context = match url_context {
        Some(url_context) => match url_context.cast::<PyUrlContext>() {
            Ok(url_context) => url_context.get().inner.clone(),
            Err(_) => clone_capsule_attr(url_context, URL_CONTEXT_CAPSULE_NAME)?,
        },
        None => UrlContext::new(),
    };

    compile_service_template(url, &url_context, store, &directory)
        .into_py()
        .and_then(|id| Bound::new(py, PyID::from(id)))
}

fn compile_service_template(
    url: String,
    url_context: &UrlContextRef,
    store: StoreRef,
    directory: &Directory,
) -> Result<ID, Problem> {
    let mut problems = Problems::default();

    let source_id = url_to_source_id(url, url_context, &mut problems)?;

    let mut catalog = Catalog::default();
    give_unwrap!(tosca_2_0::Dialect::add_to_catalog::<WithAnnotations>(&mut catalog), &mut problems);

    let service_template = compile_tosca_to_floria_with_annotations(
        &source_id,
        url_context,
        store,
        directory,
        &mut catalog,
        &mut problems,
    )?;

    problems.check()?;

    service_template.ok_or_else(|| "did not compile".gloss())
}
