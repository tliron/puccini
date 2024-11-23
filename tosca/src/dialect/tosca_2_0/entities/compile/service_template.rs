use super::super::{
    super::{super::super::grammar::*, dialect::*},
    service_template::*,
};

use {compris::annotate::*, kutil::std::error::*};

impl<AnnotatedT> ServiceTemplate<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    /// Compile to Floria.
    pub fn compile<ErrorRecipientT>(
        &self,
        vertex_template: &mut floria::VertexTemplate,
        _errors: &mut ErrorRecipientT,
    ) -> Result<(), ToscaError<AnnotatedT>>
    where
        ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>,
    {
        vertex_template.template.metadata.set_tosca_entity_static(DIALECT_ID, SERVICE_TEMPLATE_NAME);
        vertex_template.template.metadata.set_tosca_description(self.description.as_ref());
        vertex_template.template.metadata.set_tosca_custom_metadata(&self.metadata);

        Ok(())
    }
}
