use super::{
    super::super::{super::super::grammar::*, dialect::*, entities::*},
    plugin::*,
};

use {compris::annotate::*, kutil::std::error::*};

//
// GetFloriaPlugin
//

/// Get Floria plugin.
pub trait GetFloriaPlugin {
    /// Get Floria plugin.
    fn floria_plugin(
        &self,
        context: &mut CompilationContext<'_>,
    ) -> Result<Option<Plugin>, ToscaError<WithAnnotations>>;
}

impl<AnnotatedT> GetFloriaPlugin for ArtifactDefinition<AnnotatedT>
where
    AnnotatedT: 'static + Annotated + Clone + Default,
{
    fn floria_plugin(
        &self,
        context: &mut CompilationContext<'_>,
    ) -> Result<Option<Plugin>, ToscaError<WithAnnotations>> {
        let (artifact_type, _source) = must_unwrap_give!(
            context.catalog.entity::<ArtifactType<AnnotatedT>, _>(ARTIFACT_TYPE, &self.type_name, context.source_id),
            context.errors.with_fallback_annotations_from_field(self, "type_name")
        );

        // TODO: more than internal
        Ok(if artifact_type.internal {
            let global = self.properties.get_boolean_value_assignment("global").unwrap_or_default();
            let function = self.properties.get_text_value_assignment("function");
            let event = self.properties.get_text_value_assignment("event");
            Some(Plugin::new(self.file.clone(), global, function, event))
        } else {
            None
        })
    }
}

impl<AnnotatedT> GetFloriaPlugin for ImplementationDefinition<AnnotatedT>
where
    AnnotatedT: 'static + Annotated + Clone + Default,
{
    fn floria_plugin(
        &self,
        context: &mut CompilationContext<'_>,
    ) -> Result<Option<Plugin>, ToscaError<WithAnnotations>> {
        match &self.primary {
            Some(artifact) => match artifact {
                ImplementationDefinitionArtifact::Definition(artifact_definition) => {
                    artifact_definition.floria_plugin(context)
                }
                ImplementationDefinitionArtifact::Name(_) => Ok(None),
            },
            None => Ok(None),
        }
    }
}

impl<AnnotatedT> GetFloriaPlugin for OperationAssignment<AnnotatedT>
where
    AnnotatedT: 'static + Annotated + Clone + Default,
{
    fn floria_plugin(
        &self,
        context: &mut CompilationContext<'_>,
    ) -> Result<Option<Plugin>, ToscaError<WithAnnotations>> {
        match &self.implementation {
            Some(implementation_definition) => implementation_definition.floria_plugin(context),
            None => Ok(None),
        }
    }
}

impl<AnnotatedT> GetFloriaPlugin for FunctionSignature<AnnotatedT>
where
    AnnotatedT: 'static + Annotated + Clone + Default,
{
    fn floria_plugin(
        &self,
        context: &mut CompilationContext<'_>,
    ) -> Result<Option<Plugin>, ToscaError<WithAnnotations>> {
        match &self.implementation {
            Some(implementation_definition) => implementation_definition.floria_plugin(context),
            None => Ok(None),
        }
    }
}

impl<AnnotatedT> GetFloriaPlugin for FunctionDefinition<AnnotatedT>
where
    AnnotatedT: 'static + Annotated + Clone + Default,
{
    fn floria_plugin(
        &self,
        context: &mut CompilationContext<'_>,
    ) -> Result<Option<Plugin>, ToscaError<WithAnnotations>> {
        match self.signatures.first() {
            Some(signature) => signature.floria_plugin(context),
            None => Ok(None),
        }
    }
}
