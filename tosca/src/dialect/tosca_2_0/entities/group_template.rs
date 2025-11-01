use super::{
    super::{super::super::grammar::*, dialect::*},
    group_type::*,
    value::*,
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
// GroupTemplate
//

/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
///
/// Collections of nodes in a service template may be grouped together using a group definition in
/// that same service template. A group definition defines a logical grouping of node templates for
/// purposes of uniform application of policies.
///
/// Puccini note: Though this is called a "definition" in the TOSCA spec, it is actually used as a
/// template.
#[derive(Clone, Debug, Default, Depict, Resolve)]
#[depict(tag = tag::source_and_span)]
#[resolve(annotated_parameter=AnnotatedT)]
pub struct GroupTemplate<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    /// The mandatory name of the group type the group definition is based upon.
    #[resolve(required, key = "type")]
    #[depict(as(depict))]
    pub type_name: FullName,

    /// The optional description for the group definition.
    #[resolve]
    #[depict(option, style(string))]
    pub description: Option<ByteString>,

    /// Defines a section used to declare additional information.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub metadata: Metadata<AnnotatedT>,

    /// An optional map of property value assignments for the group definition.
    #[resolve]
    #[depict(iter(kv), as(depict), key_as(display), key_style(name))]
    pub properties: ValueAssignments<AnnotatedT>,

    /// An optional map of attribute value assignments for the group definition.
    #[resolve]
    #[depict(iter(kv), as(depict), key_as(display), key_style(name))]
    pub attributes: ValueAssignments<AnnotatedT>,

    /// The optional list of one or more node template names that are members of this group
    /// definition.
    #[resolve]
    #[depict(iter(item), as(depict))]
    pub members: Vec<Name>,

    #[resolve(annotations)]
    #[depict(skip)]
    pub(crate) annotations: StructAnnotations,

    #[depict(skip)]
    completion_state: CompletionState,
}

impl<AnnotatedT> Entity for GroupTemplate<AnnotatedT>
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
        _derivation_path: &mut DerivationPath,
        errors: ToscaErrorRecipientRef,
    ) -> Result<(), ToscaError<WithAnnotations>> {
        assert!(self.completion_state == CompletionState::Incomplete);
        self.completion_state = CompletionState::Cannot;

        let errors = &mut errors.to_error_recipient();

        let (group_type, type_scope) =
            entity_from_name_field!(GROUP_TYPE, GroupType, self, type_name, catalog, source_id, errors);

        complete_subentity_map_field!(
            property, properties, type_scope, self, group_type, true, catalog, source_id, errors
        );
        complete_subentity_map_field!(
            attribute, attributes, type_scope, self, group_type, true, catalog, source_id, errors
        );

        if let Some(group_type) = group_type {
            validate_entities_types(&self.members, &group_type.members, catalog, errors)?;
        }

        self.completion_state = CompletionState::Complete;
        Ok(())
    }
}

//
// GroupTemplates
//

/// Map of [GroupTemplate].
pub type GroupTemplates<AnnotatedT> = BTreeMap<Name, GroupTemplate<AnnotatedT>>;
