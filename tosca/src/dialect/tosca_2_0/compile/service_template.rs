use super::super::{super::super::grammar::*, dialect::*, entities::*};

use compris::annotate::*;

impl<AnnotatedT> ServiceTemplate<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    /// Compile to Floria.
    pub fn compile(
        &self,
        vertex_template: &mut floria::VertexTemplate,
        _context: &mut CompilationContext<'_>,
    ) -> Result<(), ToscaError<WithAnnotations>> {
        vertex_template.template.metadata.set_tosca_entity_static(DIALECT_ID, SERVICE_TEMPLATE_NAME);
        vertex_template.template.metadata.set_tosca_description(self.description.as_ref());
        vertex_template.template.metadata.set_tosca_custom_metadata(&self.metadata);

        Ok(())
    }
}
