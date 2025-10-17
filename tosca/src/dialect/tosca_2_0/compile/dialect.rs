use super::super::{
    super::super::grammar::*,
    dialect::{Dialect as Dialect2_0, *},
    entities::*,
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

        let source =
            unwrap_or_give_and_return!(context.catalog.get_source(context.source_id), context.errors, Ok(None));

        let mut id = floria::ID::new(floria::EntityKind::VertexTemplate, context.directory.clone());
        unwrap_or_give_and_return!(context.store.create_id(&mut id), context.errors, Ok(None));
        let mut floria_service_template = floria::VertexTemplate::new_with(id, None);

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
        for (entity_kind, full_name, entity_source_id) in source.namespace() {
            match entity_kind {
                ARTIFACT_TYPE => compile_type::<ArtifactType<_>, _>(
                    ARTIFACT_TYPE,
                    &full_name,
                    &entity_source_id,
                    self,
                    DIALECT_ID,
                    context,
                )?,

                CAPABILITY_TYPE => compile_type::<CapabilityType<_>, _>(
                    CAPABILITY_TYPE,
                    &full_name,
                    &entity_source_id,
                    self,
                    DIALECT_ID,
                    context,
                )?,

                DATA_TYPE => {
                    compile_type::<DataType<_>, _>(DATA_TYPE, &full_name, &entity_source_id, self, DIALECT_ID, context)?
                }

                GROUP_TYPE => compile_type::<GroupType<_>, _>(
                    GROUP_TYPE,
                    &full_name,
                    &entity_source_id,
                    self,
                    DIALECT_ID,
                    context,
                )?,

                INTERFACE_TYPE => compile_type::<InterfaceType<_>, _>(
                    INTERFACE_TYPE,
                    &full_name,
                    &entity_source_id,
                    self,
                    DIALECT_ID,
                    context,
                )?,

                NODE_TYPE => {
                    compile_type::<NodeType<_>, _>(NODE_TYPE, &full_name, &entity_source_id, self, DIALECT_ID, context)?
                }

                POLICY_TYPE => compile_type::<PolicyType<_>, _>(
                    POLICY_TYPE,
                    &full_name,
                    &entity_source_id,
                    self,
                    DIALECT_ID,
                    context,
                )?,

                RELATIONSHIP_TYPE => compile_type::<RelationshipType<_>, _>(
                    RELATIONSHIP_TYPE,
                    &full_name,
                    &entity_source_id,
                    self,
                    DIALECT_ID,
                    context,
                )?,

                _ => {}
            }
        }

        let floria_service_template_id = floria_service_template.template.id.clone();

        let group_template_kind_name = self.implementation.entity_kinds.represent(GROUP_TEMPLATE);
        let node_template_kind_name = self.implementation.entity_kinds.represent(NODE_TEMPLATE);
        let policy_template_kind_name = self.implementation.entity_kinds.represent(POLICY_TEMPLATE);

        for (entity_kind, name) in source.entity_names() {
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
                            let floria_group_template = floria::Class::new_for(context.directory.clone(), name.into());

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
                            let mut floria_node_template = floria::VertexTemplate::new_for(
                                context.directory.clone(),
                                name.clone().into(),
                                Some(floria_service_template_id.clone()),
                            );

                            node_template.compile(&mut floria_node_template, name.into(), context)?;

                            // Capabilities
                            for (name, capability) in &node_template.capabilities {
                                let mut floria_capability = floria::VertexTemplate::new_for(
                                    context.directory.clone(),
                                    name.clone(),
                                    Some(floria_node_template.template.id.clone()),
                                );

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

                                let mut floria_requirement = floria::EdgeTemplate::new_for(
                                    context.directory.clone(),
                                    name.clone(),
                                    floria_node_template.template.id.clone(),
                                    node_selector,
                                );

                                requirement.compile(&mut floria_requirement, name.clone(), context)?;

                                unwrap_or_give_and_return!(
                                    context.store.add_edge_template(floria_requirement),
                                    context.errors,
                                    Ok(None)
                                );
                            }

                            // Artifacts
                            for (name, artifact) in &node_template.artifacts {
                                let mut floria_artifact = floria::VertexTemplate::new_for(
                                    context.directory.clone(),
                                    name.clone(),
                                    Some(floria_node_template.template.id.clone()),
                                );

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
                            let floria_policy_template = floria::VertexTemplate::new_for(
                                context.directory.clone(),
                                name.into(),
                                Some(floria_service_template_id.clone()),
                            );

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
