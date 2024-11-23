use super::super::{
    super::{super::super::grammar::*, dialect::*},
    artifact_assignment::*,
};

use {compris::annotate::*, kutil::std::error::*};

impl<AnnotatedT> ArtifactAssignment<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    /// Compile to Floria.
    pub fn compile<ErrorRecipientT>(
        &self,
        vertex_template: &mut floria::VertexTemplate,
        directory: &floria::Directory,
        store: floria::StoreRef,
        errors: &mut ErrorRecipientT,
    ) -> Result<(), ToscaError<AnnotatedT>>
    where
        ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>,
    {
        vertex_template.template.class_ids.add_tosca_type(&self.type_name, directory, store.clone(), errors)?;

        vertex_template.template.metadata.set_tosca_entity_static(DIALECT_ID, ARTIFACT_NAME);
        vertex_template.template.metadata.set_tosca_description(self.description.as_ref());
        vertex_template.template.metadata.set_tosca_version(self.artifact_version.as_ref());
        vertex_template.template.metadata.set_tosca_custom_metadata(&self.metadata);

        for (name, value_assignment) in &self.properties {
            vertex_template
                .template
                .property_templates
                .insert(name.clone(), value_assignment.compile(PROPERTY_NAME, true, directory, store.clone(), errors)?);
        }

        Ok(())
    }
}
