use super::{super::super::errors::*, command::*};

use {
    compris::normal::{Map, *},
    floria::{plugins::*, *},
    kutil::std::error::*,
    read_url::*,
    std::collections::*,
};

// TODO:
// TOSCA inputs
// TOSCA outputs
// call operation

impl Compile {
    /// Instantiate.
    pub fn instantiate<StoreT, AnnotatedT>(
        &self,
        service_template_id: &ID,
        inputs: Option<Map<AnnotatedT>>,
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

        let mut floria_instance = floria_service_template.instantiate(
            &directory,
            None,
            self.instantiation_payload(inputs).as_ref(),
            &mut context,
            errors,
        )?;

        for event in self.events() {
            floria_instance.handle_event(&event, None, &mut Propagation::outgoing_all(), &mut context, errors)?;
        }

        Ok(Some(floria_instance))
    }

    fn instantiation_payload<AnnotatedT>(&self, inputs: Option<Map<AnnotatedT>>) -> Option<Expression> {
        let Some(inputs) = inputs else {
            return None;
        };

        let mut tosca = BTreeMap::default();
        tosca.insert("inputs".into(), Variant::from(inputs).into());

        let mut payload = BTreeMap::default();
        payload.insert("tosca".into(), tosca.into());

        Some(payload.into())
    }

    fn events(&self) -> Vec<String> {
        let mut length = self.events.len();
        if self.update {
            length += 1;
        }
        let mut events = Vec::with_capacity(length);
        if self.update {
            events.push(UPDATE_EVENT.into());
        }
        events.extend_from_slice(&self.events);
        events
    }
}
