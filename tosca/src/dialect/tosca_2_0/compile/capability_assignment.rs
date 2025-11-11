use super::{
    super::{super::super::grammar::*, dialect::*, entities::*},
    value_assignment::*,
};

use compris::annotate::*;

impl<AnnotatedT> CapabilityAssignment<AnnotatedT>
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
        vertex_template.template.class_ids.add_tosca_type(
            CAPABILITY_TYPE,
            CAPABILITY_TYPE_NAME,
            &self.type_name,
            context,
        )?;

        vertex_template.template.metadata.set_tosca_entity_static(DIALECT_ID, CAPABILITY_NAME);
        vertex_template.template.metadata.set_tosca_name(name);
        vertex_template.template.metadata.set_tosca_description(self.description.as_ref());
        vertex_template.template.metadata.set_tosca_custom_metadata(&self.metadata);

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

    // /// Compile to Floria.
    // pub fn compile_to_floria<StoreT, ErrorReceiverT>(
    //     &self,
    //     context: CompileToFloriaContext<OldCatalog<'_, AnnotatedT>, StoreT>,
    //     capability_name: &str,
    //     node_template_id: floria::ID,
    //     node_type: &NodeType<AnnotatedT>,
    //     errors: &mut ErrorReceiverT,
    // ) -> Result<Option<floria::ID>, ToscaError<AnnotatedT>>
    // where
    //     StoreT: floria::Store,
    //     ErrorReceiverT: ErrorReceiver<ToscaError<AnnotatedT>>,
    // {
    //     let mut floria_node_template = floria::NodeTemplate::new_for(
    //         context.floria_directory.clone(),
    //         capability_name.into(),
    //         Some(node_template_id),
    //     );

    //     floria_node_template.template.metadata.set_tosca_entity("CapabilityAssignment");
    //     floria_node_template.template.metadata.set_tosca_directives(&self.directives);

    //     match node_type.capabilities.get(capability_name) {
    //         Some(capability_definition) => {
    //             // Properties
    //             floria_node_template.template.property_templates = self.properties.compile_to_floria_as_properties(
    //                 &capability_definition.properties,
    //                 context.catalog,
    //                 context.index,
    //                 errors,
    //             )?;

    //             // Attributes
    //             floria_node_template.template.property_templates.extend(
    //                 self.attributes.compile_to_floria_as_attributes(
    //                     &capability_definition.attributes,
    //                     context.catalog,
    //                     context.index,
    //                     errors,
    //                 )?,
    //             );

    //             context.catalog.capability_types.add_floria_group_ids(
    //                 &mut floria_node_template.template.group_ids,
    //                 &"capability".into(),
    //                 context.index.index.get(&capability_definition.type_name).unwrap(),
    //             );
    //         }

    //         None => tracing::warn!("capability definition not found: {}", capability_name),
    //     }

    //     let id = floria_node_template.template.id.clone();
    //     must_unwrap_give!(context.store.add_node_template(floria_node_template), errors);
    //     Ok(Some(id))
    // }
}
