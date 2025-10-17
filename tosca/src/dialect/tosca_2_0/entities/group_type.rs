use super::{
    super::{super::super::grammar::*, data::*, dialect::*},
    attribute_definition::*,
    property_definition::*,
};

use {
    compris::{annotate::*, resolve::*},
    kutil::{
        cli::depict::*,
        std::{error::*, immutable::*},
    },
    std::collections::*,
};

/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
///
/// As with most TOSCA entities, groups are typed. A group type definition is a type of TOSCA type
/// definition and as a result supports the common keynames listed in the section Common Keynames
/// in Type Definitions.
#[derive(Clone, Debug, Default, Depict, Resolve)]
#[depict(tag = tag::source_and_span)]
#[resolve(annotated_parameter=AnnotatedT)]
pub struct GroupType<AnnotatedT>
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

    ///	An optional map of property definitions for the group type.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub properties: PropertyDefinitions<AnnotatedT>,

    /// An optional map of attribute definitions for the group type.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub attributes: AttributeDefinitions<AnnotatedT>,

    /// An optional list of one or more names of node types that are valid (allowed) as
    /// members of the group type.
    #[resolve]
    #[depict(option, iter(item), as(depict))]
    pub members: Option<Vec<FullName>>,

    #[resolve(annotations)]
    #[depict(skip)]
    pub(crate) annotations: StructAnnotations,

    #[depict(skip)]
    completion_state: CompletionState,
}

impl_type_entity!(GroupType);

impl<AnnotatedT> Entity for GroupType<AnnotatedT>
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
            GROUP_TYPE,
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
        complete_subentity_map_field!(
            attribute,
            attributes,
            parent_scope,
            self,
            parent,
            false,
            catalog,
            source_id,
            errors
        );

        if let Some(parent) = parent {
            errors_with_fallback_annotations_from_field!(
                errors, self, "members",
                complete_type_list(&mut self.members, &parent.members, catalog, source_id, errors)?;
            );
        }

        self.completion_state = CompletionState::Complete;
        Ok(())
    }
}

//
// GroupTypes
//

/// Map of [GroupType].
pub type GroupTypes<AnnotatedT> = BTreeMap<Name, GroupType<AnnotatedT>>;
