use super::{super::super::errors::*, command::*};

use {
    floria::{plugins::*, *},
    kutil::std::error::*,
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
        store: &StoreT,
        errors: &mut Errors<FloriaError>,
    ) -> Result<Option<Vertex>, MainError>
    where
        StoreT: 'static + Clone + Send + Store,
    {
        tracing::info!(directory = directory.to_string(), template = service_template_id.to_string(), "instantiating");

        let floria_service_template = store
            .get_vertex_template(service_template_id)?
            .ok_or_else(|| StoreError::ID(service_template_id.to_string()))?;

        let environment = Environment::new(self.plugin_debug)?;
        let mut library = self.library(environment, store.clone())?;

        let mut floria_instance = floria_service_template.instantiate(&directory, None, &mut library, errors)?;

        if self.update {
            floria_instance.update(&mut Propagation::outgoing_all(), &mut library, errors)?;
        }

        Ok(Some(floria_instance))
    }
}
