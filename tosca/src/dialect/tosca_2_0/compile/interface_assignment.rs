use super::super::{super::super::grammar::*, dialect::*, entities::*};

use compris::annotate::*;

impl<AnnotatedT> InterfaceAssignment<AnnotatedT>
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
            INTERFACE_TYPE,
            INTERFACE_TYPE_NAME,
            &self.type_name,
            context,
        )?;

        vertex_template.template.metadata.set_tosca_entity_static(DIALECT_ID, INTERFACE_NAME);
        vertex_template.template.metadata.set_tosca_description(self.description.as_ref());
        vertex_template.template.metadata.set_tosca_custom_metadata(&self.metadata);

        for (name, value_assignment) in &self.inputs {
            vertex_template
                .template
                .property_templates
                .insert(name.clone().into(), value_assignment.compile(PARAMETER_NAME, true, context)?);
        }

        for (name, operation_assignment) in &self.operations {
            operation_assignment.compile(vertex_template, &name, context)?;
        }

        Ok(())
    }
}
