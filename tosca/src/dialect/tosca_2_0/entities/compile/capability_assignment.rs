use super::{
    super::{
        super::{super::super::grammar::*, dialect::*},
        capability_assignment::*,
    },
    value_assignment::*,
};

use {
    compris::annotate::*,
    kutil::std::{error::*, immutable::*},
};

impl<AnnotatedT> CapabilityAssignment<AnnotatedT>
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
        // TODO: get type name?
        // floria_node_template.template.group_ids.add_tosca_type(
        //     &self.type_name,
        //     floria_directory,
        //     floria_store.clone(),
        //     errors,
        // )?;

        vertex_template.template.metadata.set_tosca_entity_static(DIALECT_ID, CAPABILITY_NAME);
        vertex_template.template.metadata.set_tosca_name(name);
        // vertex_template.template.metadata.set_tosca_description(self.description.as_ref());
        // vertex_template.template.metadata.merge_tosca_metadata(&self.metadata);

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

    // /// Compile to Floria.
    // pub fn compile_to_floria<StoreT, ErrorRecipientT>(
    //     &self,
    //     context: CompileToFloriaContext<OldCatalog<'_, AnnotatedT>, StoreT>,
    //     capability_name: &str,
    //     node_template_id: floria::ID,
    //     node_type: &NodeType<AnnotatedT>,
    //     errors: &mut ErrorRecipientT,
    // ) -> Result<Option<floria::ID>, ToscaError<AnnotatedT>>
    // where
    //     StoreT: floria::Store,
    //     ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>,
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
    //     unwrap_or_give_and_return!(context.store.add_node_template(floria_node_template), errors, Ok(None));
    //     Ok(Some(id))
    // }
}
