use super::{
    super::{
        super::{super::super::grammar::*, dialect::*},
        node_template::*,
    },
    value_assignment::*,
};

use {
    compris::annotate::*,
    kutil::std::{error::*, immutable::*},
};

impl<AnnotatedT> NodeTemplate<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    /// Compile to Floria.
    pub fn compile<ErrorRecipientT>(
        &self,
        vertex_template: &mut floria::VertexTemplate,
        name: ByteString,
        directory: &floria::Directory,
        store: floria::StoreRef,
        errors: &mut ErrorRecipientT,
    ) -> Result<(), ToscaError<AnnotatedT>>
    where
        ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>,
    {
        if let Some(type_name) = &self.type_name {
            vertex_template.template.class_ids.add_tosca_type(type_name, directory, store.clone(), errors)?;
        }

        vertex_template.template.metadata.set_tosca_entity_static(DIALECT_ID, NODE_TEMPLATE_NAME);
        vertex_template.template.metadata.set_tosca_name(name);
        vertex_template.template.metadata.set_tosca_description(self.description.as_ref());
        vertex_template.template.metadata.set_tosca_custom_metadata(&self.metadata);
        vertex_template.template.metadata.set_tosca_directives(&self.directives);

        compile_value_assignments(
            &mut vertex_template.template.property_templates,
            &self.properties,
            PROPERTY_NAME,
            true,
            directory,
            store.clone(),
            errors,
        )?;

        // TODO: name collisions?

        compile_value_assignments(
            &mut vertex_template.template.property_templates,
            &self.attributes,
            ATTRIBUTE_NAME,
            false,
            directory,
            store.clone(),
            errors,
        )?;

        Ok(())
    }
}
