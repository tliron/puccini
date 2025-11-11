use super::{super::super::errors::*, command::*};

use {
    floria::{plugins::*, *},
    kutil::std::error::*,
    read_url::*,
};

// TODO:
// TOSCA inputs
// TOSCA outputs
// call operation

impl Compile {
    /// Instantiate.
    pub fn instantiate<StoreT>(
        &self,
        service_template_id: &ID,
        directory: &Directory,
        store: StoreT,
        url_context: &UrlContextRef,
        errors: &mut Errors<FloriaError>,
    ) -> Result<Option<Vertex>, MainError>
    where
        StoreT: 'static + Clone + Send + Store,
    {
        tracing::info!(directory = directory.to_string(), template = service_template_id.to_string(), "instantiating");

        let floria_service_template = store
            .get_vertex_template(service_template_id)?
            .ok_or_else(|| StoreError::ID(service_template_id.to_string()))?;

        let environment = PluginEnvironment::new(self.plugin_debug)?;
        let mut context = self.plugin_context(environment, store.clone(), url_context.clone())?;

        let mut floria_instance = floria_service_template.instantiate(&directory, None, &mut context, errors)?;

        if self.update {
            floria_instance.update_properties(&mut Propagation::outgoing_all(), &mut context, errors)?;
        }

        Ok(Some(floria_instance))
    }
}
