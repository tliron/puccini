use super::super::{floria::*, problemo::*, readurl::*};

use {
    compris::annotate::*,
    floria::*,
    problemo::{common::*, *},
    puccini_tosca::{dialect::tosca_2_0, grammar::*},
    pyo3::prelude::*,
    read_url::*,
};

/// Compile TOSCA service template to Floria.
#[pyfunction(name = "compile_service_template")]
pub fn py_compile_service_template(
    url: Bound<'_, PyAny>,
    store: &PyStore,
    directory: Option<String>,
    url_context: Option<&PyUrlContext>,
) -> PyResult<String> {
    let url = url.to_string();

    let store = store.inner.clone();

    let directory = match directory {
        Some(directory) => directory.parse::<Directory>().into_py()?,
        None => Default::default(),
    };

    let url_context = match url_context {
        Some(url_context) => url_context.inner.clone(),
        None => UrlContext::new(),
    };

    compile_service_template(url, &url_context, store, &directory)
        .map(|id| id.to_string())
        .map_err(|problem| problem.into_py())
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
