use super::{
    super::{super::super::grammar::*, dialect::*, entities::*},
    plugin::*,
};

use {compris::annotate::*, floria::AddEventHandler, problemo::*, std::mem::*};

impl<AnnotatedT> OperationAssignment<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    /// Compile to Floria.
    pub fn compile(
        &self,
        vertex_template: &mut floria::VertexTemplate,
        name: &Name,
        context: &mut CompilationContext,
    ) -> Result<(), Problem>
    where
        AnnotatedT: 'static,
    {
        // TODO: error when clashing with interface inputs
        for (name, value_assignment) in &self.inputs {
            vertex_template
                .template
                .property_templates
                .insert(name.clone().into(), value_assignment.compile(PARAMETER_NAME, true, context)?);
        }

        if let Some(mut plugin) = self.floria_plugin(context)? {
            if let Some(event) = take(&mut plugin.event) {
                let function = take(&mut plugin.function).unwrap_or_else(|| name.clone().into());
                if let Some(plugin_id) = plugin.get_or_create(None, context)? {
                    let handler = floria::FunctionName::new(plugin_id, function)?;
                    vertex_template.template.event_handlers.add_event_handler(event, handler);
                }
            }
        }

        Ok(())
    }
}
