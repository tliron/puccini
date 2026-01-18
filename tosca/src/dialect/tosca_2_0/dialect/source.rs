use super::{
    super::{super::super::grammar::*, entities::*},
    entity_kind::*,
};

use {
    compris::{annotate::*, normal::*, resolve::*},
    problemo::*,
    std::mem::*,
};

impl super::Dialect {
    /// Initialize source.
    pub fn initialize_source<AnnotatedT, ProblemReceiverT>(
        &self,
        source: &mut Source,
        variant: Variant<AnnotatedT>,
        problems: &mut ProblemReceiverT,
    ) -> Result<(), Problem>
    where
        AnnotatedT: 'static + Annotated + Default + Clone,
        ProblemReceiverT: ProblemReceiver,
    {
        let file: Option<File<_>> = variant.resolve_with_problems(problems)?;

        let Some(file) = file else {
            return Ok(());
        };

        for import in file.imports {
            if let Some(url) = import.url {
                source.add_dependency(
                    SourceID::URL(url),
                    import.namespace.map(|namespace| namespace.into()).unwrap_or_default(),
                );
            } else if let Some(profile) = import.profile {
                source.add_dependency(
                    SourceID::Profile(profile),
                    import.namespace.map(|namespace| namespace.into()).unwrap_or_default(),
                );
            }
        }

        for (name, artifact_type) in file.artifact_types {
            give_unwrap!(source.add_entity(ARTIFACT_TYPE, name, artifact_type, false), problems);
        }

        for (name, capability_type) in file.capability_types {
            give_unwrap!(source.add_entity(CAPABILITY_TYPE, name, capability_type, false), problems);
        }

        for (name, data_type) in file.data_types {
            // Note that only data types need a fallback (to support recursive definitions)
            give_unwrap!(source.add_entity(DATA_TYPE, name.clone(), data_type, true), problems);
        }

        for (name, group_type) in file.group_types {
            give_unwrap!(source.add_entity(GROUP_TYPE, name, group_type, false), problems);
        }

        for (name, interface_type) in file.interface_types {
            give_unwrap!(source.add_entity(INTERFACE_TYPE, name, interface_type, false), problems);
        }

        for (name, node_type) in file.node_types {
            give_unwrap!(source.add_entity(NODE_TYPE, name, node_type, false), problems);
        }

        for (name, policy_type) in file.policy_types {
            give_unwrap!(source.add_entity(POLICY_TYPE, name, policy_type, false), problems);
        }

        for (name, relationship_type) in file.relationship_types {
            give_unwrap!(source.add_entity(RELATIONSHIP_TYPE, name, relationship_type, false), problems);
        }

        if let Some(mut service_template) = file.service_template {
            for (name, group_template) in take(&mut service_template.groups) {
                give_unwrap!(source.add_entity(GROUP_TEMPLATE, name, group_template, false), problems);
            }

            for (name, node_template) in take(&mut service_template.node_templates) {
                give_unwrap!(source.add_entity(NODE_TEMPLATE, name, node_template, false), problems);
            }

            let mut index = 0;
            for policy_template in take(&mut service_template.policies) {
                index += 1;
                let name = index.to_string().parse().expect("policy template name");
                give_unwrap!(source.add_entity(POLICY_TEMPLATE, name, policy_template, false), problems);
            }

            for (name, relationship_template) in take(&mut service_template.relationship_templates) {
                give_unwrap!(source.add_entity(RELATIONSHIP_TEMPLATE, name, relationship_template, false), problems);
            }

            // Note that the service template has no name!
            give_unwrap!(source.add_entity(SERVICE_TEMPLATE, Default::default(), service_template, false), problems);
        }

        for (name, repository) in file.repositories {
            give_unwrap!(source.add_entity(REPOSITORY, name, repository, false), problems);
        }

        for (name, function) in file.functions {
            give_unwrap!(source.add_entity(FUNCTION, name, function, false), problems);
        }

        Ok(())
    }
}
