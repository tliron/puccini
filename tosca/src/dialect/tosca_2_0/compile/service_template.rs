use super::{
    super::{
        super::super::grammar::*,
        dialect::{Dialect as Dialect2_0, *},
        entities::*,
    },
    plugin::*,
};

use {compris::annotate::*, kutil::std::error::*};

impl Dialect2_0 {
    /// Compile service template to Floria.
    pub fn compile_service_template<AnnotatedT>(
        &self,
        context: &mut CompilationContext<'_>,
    ) -> Result<Option<floria::ID>, ToscaError<WithAnnotations>>
    where
        AnnotatedT: 'static + Annotated + Clone + Default,
    {
        tracing::info!(
            directory = context.directory.to_string(),
            source = context.source_id.to_string(),
            "compiling service template"
        );

        get_or_create_plugin_by_url(PLUGIN_URL, Default::default(), Some(PLUGIN_NAME), context)?;

        let source = unwrap_or_give_and_return!(context.catalog.source(context.source_id), context.errors, Ok(None));

        let mut floria_service_template =
            floria::VertexTemplate::new_with_name(context.directory.clone(), SERVICE_TEMPLATE_NAME.into(), None)?;

        let service_template_kind_name = self.implementation.entity_kinds.represent(SERVICE_TEMPLATE);

        match source.entity::<ServiceTemplate<AnnotatedT>, _>(
            SERVICE_TEMPLATE,
            &service_template_kind_name,
            &Default::default(),
        ) {
            Ok(service_template) => service_template.compile(&mut floria_service_template, context)?,
            Err(ToscaError::Undeclared(_)) => return Ok(None),
            Err(error) => context.errors.give(error)?,
        }

        // Types

        let type_compiler = &mut TypeEntityCompiler::new(self, DIALECT_ID, context);

        for (entity_kind, full_name, source_id) in source.canonical_namespace() {
            match entity_kind {
                ARTIFACT_TYPE => {
                    type_compiler.compile::<ArtifactType<_>>(entity_kind, ARTIFACT_TYPE_NAME, &full_name, &source_id)?
                }

                CAPABILITY_TYPE => type_compiler.compile::<CapabilityType<_>>(
                    entity_kind,
                    CAPABILITY_TYPE_NAME,
                    &full_name,
                    &source_id,
                )?,

                DATA_TYPE => {
                    type_compiler.compile::<DataType<_>>(entity_kind, DATA_TYPE_NAME, &full_name, &source_id)?
                }

                GROUP_TYPE => {
                    type_compiler.compile::<GroupType<_>>(entity_kind, GROUP_TYPE_NAME, &full_name, &source_id)?
                }

                INTERFACE_TYPE => type_compiler.compile::<InterfaceType<_>>(
                    entity_kind,
                    INTERFACE_TYPE_NAME,
                    &full_name,
                    &source_id,
                )?,

                NODE_TYPE => {
                    type_compiler.compile::<NodeType<_>>(entity_kind, NODE_TYPE_NAME, &full_name, &source_id)?
                }

                POLICY_TYPE => {
                    type_compiler.compile::<PolicyType<_>>(entity_kind, POLICY_TYPE_NAME, &full_name, &source_id)?
                }

                RELATIONSHIP_TYPE => type_compiler.compile::<RelationshipType<_>>(
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

        let mut entity_names = source.entity_names();
        entity_names.sort();
        for (entity_kind, name) in entity_names {
            match entity_kind {
                GROUP_TEMPLATE => {
                    tracing::debug!(
                        source = context.source_id.to_string(),
                        name = name.to_string(),
                        type = GROUP_TEMPLATE_NAME,
                        "compiling"
                    );

                    match source.entity::<GroupTemplate<AnnotatedT>, _>(
                        GROUP_TEMPLATE,
                        &group_template_kind_name,
                        &name,
                    ) {
                        Ok(_group_template) => {
                            let floria_group_template = floria::Class::new_with_name(
                                context.directory.clone(),
                                to_floria_id_name(GROUP_TEMPLATE_NAME, name.as_ref()),
                            )?;

                            // TODO

                            unwrap_or_give_and_return!(
                                context.store.add_class(floria_group_template),
                                context.errors,
                                Ok(None)
                            );
                        }

                        Err(error) => context.errors.give(error)?,
                    }
                }

                NODE_TEMPLATE => {
                    tracing::debug!(
                        source = context.source_id.to_string(),
                        name = name.to_string(),
                        type = NODE_TEMPLATE_NAME,
                        "compiling"
                    );

                    match source.entity::<NodeTemplate<AnnotatedT>, _>(NODE_TEMPLATE, &node_template_kind_name, &name) {
                        Ok(node_template) => {
                            let floria_id = to_floria_id_name(NODE_TEMPLATE_NAME, name.as_ref());

                            let mut floria_node_template = floria::VertexTemplate::new_with_name(
                                context.directory.clone(),
                                floria_id.clone(),
                                Some(floria_service_template_id.clone()),
                            )?;

                            node_template.compile(&mut floria_node_template, name.into(), context)?;

                            // Capabilities
                            for (name, capability) in &node_template.capabilities {
                                let mut floria_capability = floria::VertexTemplate::new_with_name(
                                    context.directory.clone(),
                                    to_floria_id_name_contained(CAPABILITY_NAME, name, &floria_id),
                                    Some(floria_node_template.template.id.clone()),
                                )?;

                                capability.compile(&mut floria_capability, name.clone(), context)?;

                                floria_node_template
                                    .contained_vertex_template_ids
                                    .push(floria_capability.template.id.clone());

                                unwrap_or_give_and_return!(
                                    context.store.add_vertex_template(floria_capability),
                                    context.errors,
                                    Ok(None)
                                );
                            }

                            // Requirements
                            for (name, requirement) in &node_template.requirements {
                                // TODO
                                let node_selector =
                                    floria::VertexSelector::new_vertex(floria_node_template.template.id.clone());

                                let mut floria_requirement = floria::EdgeTemplate::new_with_name(
                                    context.directory.clone(),
                                    to_floria_id_name_contained(REQUIREMENT_NAME, name, &floria_id),
                                    floria_node_template.template.id.clone(),
                                    node_selector,
                                )?;

                                requirement.compile(&mut floria_requirement, name.clone(), context)?;

                                unwrap_or_give_and_return!(
                                    context.store.add_edge_template(floria_requirement),
                                    context.errors,
                                    Ok(None)
                                );
                            }

                            // Interfaces
                            for (name, interface) in &node_template.interfaces {
                                let mut floria_interface = floria::VertexTemplate::new_with_name(
                                    context.directory.clone(),
                                    to_floria_id_name_contained(INTERFACE_NAME, name, &floria_id),
                                    Some(floria_node_template.template.id.clone()),
                                )?;

                                interface.compile(&mut floria_interface, context)?;

                                floria_node_template
                                    .contained_vertex_template_ids
                                    .push(floria_interface.template.id.clone());

                                unwrap_or_give_and_return!(
                                    context.store.add_vertex_template(floria_interface),
                                    context.errors,
                                    Ok(None)
                                );
                            }

                            // Artifacts
                            for (name, artifact) in &node_template.artifacts {
                                let mut floria_artifact = floria::VertexTemplate::new_with_name(
                                    context.directory.clone(),
                                    to_floria_id_name_contained(ARTIFACT_NAME, name, &floria_id),
                                    Some(floria_node_template.template.id.clone()),
                                )?;

                                artifact.compile(&mut floria_artifact, context)?;

                                floria_node_template
                                    .contained_vertex_template_ids
                                    .push(floria_artifact.template.id.clone());

                                unwrap_or_give_and_return!(
                                    context.store.add_vertex_template(floria_artifact),
                                    context.errors,
                                    Ok(None)
                                );
                            }

                            floria_service_template
                                .contained_vertex_template_ids
                                .push(floria_node_template.template.id.clone());

                            unwrap_or_give_and_return!(
                                context.store.add_vertex_template(floria_node_template),
                                context.errors,
                                Ok(None)
                            );
                        }

                        Err(error) => context.errors.give(error)?,
                    }
                }

                POLICY_TEMPLATE => {
                    tracing::debug!(
                        source = context.source_id.to_string(),
                        name = name.to_string(),
                        type = POLICY_TEMPLATE_NAME,
                        "compiling"
                    );

                    match source.entity::<PolicyTemplate<WithAnnotations>, _>(
                        POLICY_TEMPLATE,
                        &policy_template_kind_name,
                        &name,
                    ) {
                        Ok(_policy_template) => {
                            let floria_policy_template = floria::VertexTemplate::new_with_name(
                                context.directory.clone(),
                                to_floria_id_name(POLICY_TEMPLATE_NAME, name.as_ref()),
                                Some(floria_service_template_id.clone()),
                            )?;

                            // TODO

                            floria_service_template
                                .contained_vertex_template_ids
                                .push(floria_policy_template.template.id.clone());

                            unwrap_or_give_and_return!(
                                context.store.add_vertex_template(floria_policy_template),
                                context.errors,
                                Ok(None)
                            );
                        }

                        Err(error) => context.errors.give(error)?,
                    }
                }

                _ => {}
            }
        }

        unwrap_or_give_and_return!(
            context.store.add_vertex_template(floria_service_template),
            context.errors,
            Ok(None)
        );

        Ok(Some(floria_service_template_id.clone()))
    }
}

impl<AnnotatedT> ServiceTemplate<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    /// Compile to Floria.
    pub fn compile(
        &self,
        vertex_template: &mut floria::VertexTemplate,
        _context: &mut CompilationContext<'_>,
    ) -> Result<(), ToscaError<WithAnnotations>> {
        vertex_template.template.metadata.set_tosca_entity_static(DIALECT_ID, SERVICE_TEMPLATE_NAME);
        vertex_template.template.metadata.set_tosca_description(self.description.as_ref());
        vertex_template.template.metadata.set_tosca_custom_metadata(&self.metadata);

        Ok(())
    }
}
