use super::super::{super::super::grammar::*, dialect::*, entities::*};

use compris::annotate::*;

impl<AnnotatedT> ArtifactAssignment<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    /// Compile to Floria.
    pub fn compile(
        &self,
        vertex_template: &mut floria::VertexTemplate,
        context: &mut CompilationContext<'_>,
    ) -> Result<(), ToscaError<WithAnnotations>>
    where
        AnnotatedT: 'static,
    {
        vertex_template.template.class_ids.add_tosca_type(
            ARTIFACT_TYPE,
            ARTIFACT_TYPE_NAME,
            &self.type_name,
            context,
        )?;

        vertex_template.template.metadata.set_tosca_entity_static(DIALECT_ID, ARTIFACT_NAME);
        vertex_template.template.metadata.set_tosca_description(self.description.as_ref());
        vertex_template.template.metadata.set_tosca_version(self.artifact_version.as_ref());
        vertex_template.template.metadata.set_tosca_custom_metadata(&self.metadata);

        for (name, value_assignment) in &self.properties {
            vertex_template
                .template
                .property_templates
                .insert(name.clone().into(), value_assignment.compile(PROPERTY_NAME, true, context)?);
        }

        Ok(())
    }
}
