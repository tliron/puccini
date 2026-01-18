use super::{
    super::{
        super::super::grammar::*,
        dialect::{Dialect as Dialect2_0, *},
        entities::*,
    },
    plugin::*,
    value_assignment::*,
};

use {compris::annotate::*, floria::AddEventHandler, kutil::std::immutable::*, problemo::*};

impl Dialect2_0 {
    /// Compile service template to Floria.
    pub fn compile_service_template<AnnotatedT>(
        &self,
        context: &mut CompilationContext,
    ) -> Result<Option<floria::ID>, Problem>
    where
        AnnotatedT: 'static + Annotated + Clone + Default,
    {
        tracing::info!(
            directory = context.directory.to_string(),
            source = context.source_id.to_string(),
            "compiling service template"
        );

        let source = give_unwrap!(context.catalog.source(context.source_id), &mut context.problems);

        let mut floria_service_template =
            floria::VertexTemplate::new_with_name(context.directory.clone(), SERVICE_TEMPLATE_NAME.into(), None)?;

        let service_template_kind_name = self.implementation.entity_kinds.represent(SERVICE_TEMPLATE);

        match source.entity::<ServiceTemplate<AnnotatedT>>(
            SERVICE_TEMPLATE,
            &service_template_kind_name,
            &Default::default(),
        ) {
            Ok(service_template) => service_template.compile(&mut floria_service_template, context)?,
            Err(problem) => {
                if problem.has_error_type::<UndeclaredError>() {
                    return Ok(None);
                } else {
                    context.problems.give(problem)?;
                }
            }
        }

        // Types

        let compiler = &mut TypeEntityCompiler::new(self, DIALECT_ID, context);

        for (entity_kind, full_name, source_id) in source.canonical_namespace() {
            match entity_kind {
                ARTIFACT_TYPE => compiler.compile::<ArtifactType<_>, AnnotatedT>(
                    entity_kind,
                    ARTIFACT_TYPE_NAME,
                    &full_name,
                    &source_id,
                )?,

                CAPABILITY_TYPE => compiler.compile::<CapabilityType<_>, AnnotatedT>(
                    entity_kind,
                    CAPABILITY_TYPE_NAME,
                    &full_name,
                    &source_id,
                )?,

                DATA_TYPE => {
                    compiler.compile::<DataType<_>, AnnotatedT>(entity_kind, DATA_TYPE_NAME, &full_name, &source_id)?
                }

                GROUP_TYPE => compiler.compile::<GroupType<_>, AnnotatedT>(
                    entity_kind,
                    GROUP_TYPE_NAME,
                    &full_name,
                    &source_id,
                )?,

                INTERFACE_TYPE => compiler.compile::<InterfaceType<_>, AnnotatedT>(
                    entity_kind,
                    INTERFACE_TYPE_NAME,
                    &full_name,
                    &source_id,
                )?,

                NODE_TYPE => {
                    compiler.compile::<NodeType<_>, AnnotatedT>(entity_kind, NODE_TYPE_NAME, &full_name, &source_id)?
                }

                POLICY_TYPE => compiler.compile::<PolicyType<_>, AnnotatedT>(
                    entity_kind,
                    POLICY_TYPE_NAME,
                    &full_name,
                    &source_id,
                )?,

                RELATIONSHIP_TYPE => compiler.compile::<RelationshipType<_>, AnnotatedT>(
                    entity_kind,
                    RELATIONSHIP_TYPE_NAME,
                    &full_name,
                    &source_id,
                )?,

                _ => {}
            }
        }

        let floria_service_template_id = floria_service_template.template.id.clone();

        let group_template_kind_name = self.implementation.entity_kinds.represent(GROUP_TEMPLATE);
        let node_template_kind_name = self.implementation.entity_kinds.represent(NODE_TEMPLATE);
        let policy_template_kind_name = self.implementation.entity_kinds.represent(POLICY_TEMPLATE);

        let mut entity_names: Vec<_> = source.entity_names().collect();
        entity_names.sort();
        for (entity_kind, name) in entity_names {
            match *entity_kind {
                GROUP_TEMPLATE => {
                    tracing::debug!(
                        source = context.source_id.to_string(),
                        name = name.to_string(),
                        type = GROUP_TEMPLATE_NAME,
                        "compiling"
                    );

                    if let Some(_group_template) = source
                        .entity::<GroupTemplate<AnnotatedT>>(GROUP_TEMPLATE, &group_template_kind_name, name)
                        .give_ok(&mut context.problems)?
                    {
                        let floria_group_template = floria::Class::new_with_name(
                            context.directory.clone(),
                            name.to_floria_name(GROUP_TEMPLATE_NAME),
                        )?;

                        // TODO

                        give_unwrap!(context.store.add_class(floria_group_template), &mut context.problems);
                    }
                }

                NODE_TEMPLATE => {
                    tracing::debug!(
                        source = context.source_id.to_string(),
                        name = name.to_string(),
                        type = NODE_TEMPLATE_NAME,
                        "compiling"
                    );

                    if let Some(node_template) = source
                        .entity::<NodeTemplate<AnnotatedT>>(NODE_TEMPLATE, &node_template_kind_name, name)
                        .give_ok(&mut context.problems)?
                    {
                        let floria_id = name.to_floria_name(NODE_TEMPLATE_NAME);

                        let mut floria_node_template = floria::VertexTemplate::new_with_name(
                            context.directory.clone(),
                            floria_id.clone(),
                            Some(floria_service_template_id.clone()),
                        )?;

                        node_template.compile(&mut floria_node_template, name.clone(), context)?;

                        // Capabilities
                        for (name, capability) in &node_template.capabilities {
                            let mut floria_capability = floria::VertexTemplate::new_with_name(
                                context.directory.clone(),
                                name.to_floria_name_contained(CAPABILITY_NAME, &floria_id),
                                Some(floria_node_template.template.id.clone()),
                            )?;

                            capability.compile(&mut floria_capability, name.clone(), context)?;

                            floria_node_template
                                .contained_vertex_template_ids
                                .push(floria_capability.template.id.clone());

                            give_unwrap!(context.store.add_vertex_template(floria_capability), &mut context.problems);
                        }

                        // Requirements
                        for (name, requirement) in &node_template.requirements {
                            // TODO
                            let node_selector =
                                floria::VertexSelector::new_vertex(floria_node_template.template.id.clone());

                            let mut floria_requirement = floria::EdgeTemplate::new_with_name(
                                context.directory.clone(),
                                name.to_floria_name_contained(REQUIREMENT_NAME, &floria_id),
                                floria_node_template.template.id.clone(),
                                node_selector,
                            )?;

                            requirement.compile(&mut floria_requirement, name.clone(), context)?;

                            give_unwrap!(context.store.add_edge_template(floria_requirement), &mut context.problems);
                        }

                        // Interfaces
                        for (name, interface) in &node_template.interfaces {
                            let mut floria_interface = floria::VertexTemplate::new_with_name(
                                context.directory.clone(),
                                name.to_floria_name_contained(INTERFACE_NAME, &floria_id),
                                Some(floria_node_template.template.id.clone()),
                            )?;

                            interface.compile(&mut floria_interface, context)?;

                            floria_node_template
                                .contained_vertex_template_ids
                                .push(floria_interface.template.id.clone());

                            give_unwrap!(context.store.add_vertex_template(floria_interface), &mut context.problems);
                        }

                        // Artifacts
                        for (name, artifact) in &node_template.artifacts {
                            let mut floria_artifact = floria::VertexTemplate::new_with_name(
                                context.directory.clone(),
                                name.to_floria_name_contained(ARTIFACT_NAME, &floria_id),
                                Some(floria_node_template.template.id.clone()),
                            )?;

                            artifact.compile(&mut floria_artifact, context)?;

                            floria_node_template
                                .contained_vertex_template_ids
                                .push(floria_artifact.template.id.clone());

                            give_unwrap!(context.store.add_vertex_template(floria_artifact), &mut context.problems);
                        }

                        floria_service_template
                            .contained_vertex_template_ids
                            .push(floria_node_template.template.id.clone());

                        give_unwrap!(context.store.add_vertex_template(floria_node_template), &mut context.problems);
                    }
                }

                POLICY_TEMPLATE => {
                    tracing::debug!(
                        source = context.source_id.to_string(),
                        name = name.to_string(),
                        type = POLICY_TEMPLATE_NAME,
                        "compiling"
                    );

                    if let Some(_policy_template) = source
                        .entity::<PolicyTemplate<WithAnnotations>>(POLICY_TEMPLATE, &policy_template_kind_name, name)
                        .give_ok(&mut context.problems)?
                    {
                        let floria_policy_template = floria::VertexTemplate::new_with_name(
                            context.directory.clone(),
                            name.to_floria_name(POLICY_TEMPLATE_NAME),
                            Some(floria_service_template_id.clone()),
                        )?;

                        // TODO

                        floria_service_template
                            .contained_vertex_template_ids
                            .push(floria_policy_template.template.id.clone());

                        give_unwrap!(context.store.add_vertex_template(floria_policy_template), &mut context.problems);
                    }
                }

                _ => {}
            }
        }

        give_unwrap!(context.store.add_vertex_template(floria_service_template), &mut context.problems);

        Ok(Some(floria_service_template_id.clone()))
    }
}

impl<AnnotatedT> ServiceTemplate<AnnotatedT>
where
    AnnotatedT: 'static + Annotated + Clone + Default,
{
    /// Compile to Floria.
    pub fn compile(
        &self,
        vertex_template: &mut floria::VertexTemplate,
        context: &mut CompilationContext,
    ) -> Result<(), Problem> {
        vertex_template.template.metadata.set_tosca_entity_static(DIALECT_ID, SERVICE_TEMPLATE_NAME);
        vertex_template.template.metadata.set_tosca_description(self.description.as_ref());
        vertex_template.template.metadata.set_tosca_custom_metadata(&self.metadata);

        if let Some(plugin_id) = Plugin::get_or_create_implicit(context)?
            && let Some(function_name) = floria::FunctionName::new(plugin_id, ByteString::from_static("set_inputs"))
                .give_ok(&mut context.problems)?
        {
            vertex_template.template.event_handlers.add_static_event_handler(floria::ADDED_EVENT, function_name);
        }

        // TODO:
        // We are relying on the fact that "output" > "input" in sorting!
        // This allows outputs to call $get_input

        compile_value_assignments(
            &mut vertex_template.template.property_templates,
            &self.input_assignments,
            "input",
            PARAMETER_NAME,
            true,
            context,
        )?;

        compile_value_assignments(
            &mut vertex_template.template.property_templates,
            &self.output_assignments,
            "output",
            PARAMETER_NAME,
            false,
            context,
        )?;

        Ok(())
    }
}
