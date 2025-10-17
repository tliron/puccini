use super::{
    super::{super::super::grammar::*, data::*, dialect::*},
    property_definition::*,
    trigger_definition::*,
};

use {
    compris::{annotate::*, resolve::*},
    kutil::{
        cli::depict::*,
        std::{error::*, immutable::*},
    },
    std::collections::*,
};

//
// PolicyType
//

/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
///
/// A policy type defines a type of a policy that affects or governs an application or service's
/// topology at some stage of its lifecycle but is not explicitly part of the topology itself
/// (i.e., it does not prevent the application or service from being deployed or run if it did
/// not exist).
#[derive(Clone, Debug, Default, Depict, Resolve)]
#[depict(tag = tag::source_and_span)]
#[resolve(annotated_parameter=AnnotatedT)]
pub struct PolicyType<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    /// An optional parent type name from which this type derives.
    #[resolve]
    #[depict(option, as(depict))]
    pub derived_from: Option<FullName>,

    /// An optional version for the type definition.
    #[resolve]
    #[depict(option, as(depict))]
    pub version: Option<Version>,

    /// Defines a section used to declare additional information.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub metadata: Metadata<AnnotatedT>,

    /// An optional description for the type.
    #[resolve]
    #[depict(option, style(string))]
    pub description: Option<ByteString>,

    ///	An optional map of property definitions for the policy type.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub properties: PropertyDefinitions<AnnotatedT>,

    /// An optional list of valid node types or group types the policy type can be applied to.
    #[resolve]
    #[depict(option, iter(item), as(depict))]
    pub targets: Option<Vec<FullName>>,

    /// An optional map of policy triggers for the policy type.
    #[resolve]
    #[depict(iter(kv), as(depict), key_as(display), key_style(name))]
    pub triggers: TriggerDefinitions<AnnotatedT>,

    #[resolve(annotations)]
    #[depict(skip)]
    pub(crate) annotations: StructAnnotations,

    #[depict(skip)]
    completion_state: CompletionState,
}

impl_type_entity!(PolicyType);

impl<AnnotatedT> Entity for PolicyType<AnnotatedT>
where
    AnnotatedT: 'static + Annotated + Clone + Default,
{
    fn completion_state(&self) -> CompletionState {
        self.completion_state
    }

    fn complete(
        &mut self,
        catalog: &mut Catalog,
        source_id: &SourceID,
        derivation_path: &mut DerivationPath,
        errors: ToscaErrorRecipientRef,
    ) -> Result<(), ToscaError<WithAnnotations>> {
        assert!(self.completion_state == CompletionState::Incomplete);
        self.completion_state = CompletionState::Cannot;

        let errors = &mut errors.to_error_recipient();

        let (parent, parent_scope) = entity_from_name_field_checked!(
            POLICY_TYPE,
            self,
            derived_from,
            catalog,
            source_id,
            derivation_path,
            errors
        );

        complete_subentity_map_field!(
            property,
            properties,
            parent_scope,
            self,
            parent,
            false,
            catalog,
            source_id,
            errors
        );
        complete_subentity_map_field!(trigger, triggers, parent_scope, self, parent, false, catalog, source_id, errors);

        if let Some(parent) = parent {
            errors_with_fallback_annotations_from_field!(
                errors, self, "targets",
                complete_type_list(&mut self.targets, &parent.targets, catalog, source_id, errors)?;
            );
        }

        self.completion_state = CompletionState::Complete;
        Ok(())
    }
}

//
// PolicyTypes
//

/// Map of [PolicyType].
pub type PolicyTypes<AnnotatedT> = BTreeMap<Name, PolicyType<AnnotatedT>>;
