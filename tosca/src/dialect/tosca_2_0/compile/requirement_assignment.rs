use super::{
    super::{super::super::grammar::*, dialect::*, entities::*},
    value_assignment::*,
};

use {compris::annotate::*, problemo::*};

impl<AnnotatedT> RequirementAssignment<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    /// Compile to Floria.
    pub fn compile(
        &self,
        edge_template: &mut floria::EdgeTemplate,
        name: Name,
        context: &mut CompilationContext,
    ) -> Result<(), Problem>
    where
        AnnotatedT: 'static,
    {
        edge_template.template.metadata.set_tosca_entity_static(DIALECT_ID, RELATIONSHIP_TEMPLATE_NAME);
        edge_template.template.metadata.set_tosca_name(name);
        edge_template.template.metadata.set_tosca_description(self.description.as_ref());
        edge_template.template.metadata.set_tosca_custom_metadata(&self.metadata);

        if let Some(relationship) = &self.relationship {
            compile_value_assignments(
                &mut edge_template.template.property_templates,
                &relationship.properties,
                "",
                PROPERTY_NAME,
                true,
                context,
            )?;

            // TODO: name collisions?

            compile_value_assignments(
                &mut edge_template.template.property_templates,
                &relationship.attributes,
                "",
                ATTRIBUTE_NAME,
                false,
                context,
            )?;
        }

        Ok(())
    }

    // /// Compile to Floria.
    // pub fn compile_to_floria<StoreT, ProblemReceiverT>(
    //     &self,
    //     context: CompileToFloriaContext<OldCatalog<'_, AnnotatedT>, StoreT>,
    //     requirement_name: &str,
    //     node_template_id: floria::ID,
    //     node_type: &NodeType<AnnotatedT>,
    //     errors: &mut ProblemReceiverT,
    // ) -> Result<Option<floria::ID>, ToscaError<AnnotatedT>>
    // where
    //     StoreT: floria::Store,
    //     ProblemReceiverT: ProblemReceiver<ToscaError<AnnotatedT>>,
    // {
    //     let finder = match self.new_finder(context.catalog, context.index) {
    //         Ok(finder) => finder,
    //         Err(error) => {
    //             errors.give(error)?;
    //             return Ok(None);
    //         }
    //     };

    //     let mut floria_namespace = context.floria_directory.clone();
    //     floria_namespace.push(requirement_name.into());

    //     let mut floria_relationship_template = match floria::RelationshipTemplate::new(
    //         floria_namespace.clone(),
    //         node_template_id,
    //         floria::NodeSelector::new_finder(finder),
    //         context.store,
    //     ) {
    //         Ok(floria_relationship_template) => floria_relationship_template,
    //         Err(error) => {
    //             errors.give(error)?;
    //             return Ok(None);
    //         }
    //     };

    //     floria_relationship_template.template.metadata.set_tosca_entity("RequirementAssignment");
    //     floria_relationship_template.template.metadata.set_tosca_directives(&self.directives);

    //     if let Some(relationship_assignment) = &self.relationship {
    //         match node_type.requirements.get_first(&requirement_name.into()) {
    //             Some(requirement_definition) => {
    //                 // Properties and attributes
    //                 floria_relationship_template.template.property_templates =
    //                     relationship_assignment.properties.compile_to_floria_as_properties(
    //                         &requirement_definition.relationship.properties,
    //                         context.catalog,
    //                         context.index,
    //                         errors,
    //                     )?;
    //                 floria_relationship_template.template.property_templates.extend(
    //                     relationship_assignment.attributes.compile_to_floria_as_attributes(
    //                         &requirement_definition.relationship.attributes,
    //                         context.catalog,
    //                         context.index,
    //                         errors,
    //                     )?,
    //                 );

    //                 context.catalog.relationship_types.add_floria_group_ids(
    //                     &mut floria_relationship_template.template.group_ids,
    //                     &"relationship".into(),
    //                     context.index.index.get(&requirement_definition.relationship.type_name).unwrap(),
    //                 );
    //             }

    //             None => tracing::warn!("requirement definition not found: {}", requirement_name),
    //         }
    //     }

    //     let id = floria_relationship_template.template.id.clone();
    //     if let Err(error) = context.store.add_relationship_template(floria_relationship_template) {
    //         errors.give(error)?;
    //         return Ok(None);
    //     }

    //     Ok(Some(id))
    // }

    // fn new_finder(
    //     &self,
    //     catalog: &OldCatalog<'_, AnnotatedT>,
    //     index: &Index,
    // ) -> Result<floria::Call, ToscaError<AnnotatedT>> {
    //     let mut finder = floria::Call::new(get_dispatch_name("select_capability"), Default::default());

    //     let mut argument = Map::default();

    //     if let Some(capability) = self.get_capability(catalog)? {
    //         match capability {
    //             RequirementAssignmentCapability::TypeName(type_name) => {
    //                 argument.into_insert("capability_type_name", type_name.to_string());
    //             }

    //             RequirementAssignmentCapability::Name(name) => {
    //                 argument.into_insert("capability_name", name.to_string());
    //             }
    //         }
    //     }

    //     if let Some(node) = self.get_node(catalog, index)? {
    //         match node {
    //             RequirementAssignmentNode::TypeName(type_name) => {
    //                 argument.into_insert("node_type_name", type_name.to_string());
    //             }

    //             RequirementAssignmentNode::Template((template_name, index)) => {
    //                 argument.into_insert("node_template_name", template_name.to_string());
    //                 argument.into_insert("node_template_index", index);
    //             }
    //         }
    //     }

    //     if !argument.inner.is_empty() {
    //         finder.arguments.push(floria::Expression::Literal(argument.into()));
    //     }

    //     Ok(finder)
    // }

    // fn get_capability(
    //     &self,
    //     _catalog: &OldCatalog<'_, AnnotatedT>,
    // ) -> Result<Option<RequirementAssignmentCapability>, ToscaError<AnnotatedT>> {
    //     // TODO: capability type name or capability name
    //     Ok(self.capability.as_ref().map(|name| RequirementAssignmentCapability::Name(name.name.clone())))
    // }

    // fn get_node(
    //     &self,
    //     catalog: &OldCatalog<'_, AnnotatedT>,
    //     index: &Index,
    // ) -> Result<Option<RequirementAssignmentNode>, ToscaError<AnnotatedT>> {
    //     match &self.node {
    //         Some(node) => {
    //             match node.index {
    //                 // node type name or node template name
    //                 // TODO
    //                 None => match index.index.get(&node.full_name) {
    //                     Some(node_id) => match catalog.node_types.has_id(node_id) {
    //                         true => Ok(Some(RequirementAssignmentNode::TypeName(node.full_name.clone()))),

    //                         false => Err(UnknownTypeError::new(
    //                             node.full_name.to_string(),
    //                             "RequirementAssignment.get_node".into(),
    //                         )
    //                         .into()),
    //                     },

    //                     None => Ok(None),
    //                 },

    //                 // node template name and index
    //                 Some(index) => Ok(Some(RequirementAssignmentNode::Template((node.full_name.name.clone(), index)))),
    //             }
    //         }

    //         None => Ok(None),
    //     }
    // }
}
