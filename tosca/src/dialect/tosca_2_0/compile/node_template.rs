use super::{
    super::{super::super::grammar::*, dialect::*, entities::*},
    value_assignment::*,
};

use compris::annotate::*;

impl<AnnotatedT> NodeTemplate<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    /// Compile to Floria.
    pub fn compile(
        &self,
        vertex_template: &mut floria::VertexTemplate,
        name: Name,
        context: &mut CompilationContext<'_>,
    ) -> Result<(), ToscaError<WithAnnotations>>
    where
        AnnotatedT: 'static,
    {
        if let Some(type_name) = &self.type_name {
            vertex_template.template.class_ids.add_tosca_type(NODE_TYPE, NODE_TYPE_NAME, type_name, context)?;
        }

        vertex_template.template.metadata.set_tosca_entity_static(DIALECT_ID, NODE_TEMPLATE_NAME);
        vertex_template.template.metadata.set_tosca_name(name);
        vertex_template.template.metadata.set_tosca_description(self.description.as_ref());
        vertex_template.template.metadata.set_tosca_custom_metadata(&self.metadata);
        vertex_template.template.metadata.set_tosca_directives(&self.directives);

        compile_value_assignments(
            &mut vertex_template.template.property_templates,
            &self.properties,
            "",
            PROPERTY_NAME,
            true,
            context,
        )?;

        // TODO: name collisions?

        compile_value_assignments(
            &mut vertex_template.template.property_templates,
            &self.attributes,
            "",
            ATTRIBUTE_NAME,
            false,
            context,
        )?;

        Ok(())
    }
}
