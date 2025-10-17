use super::{
    super::{super::super::grammar::*, data::*, dialect::*},
    attribute_definition::*,
    interface_definition::*,
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

//
// RelationshipType
//

/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
///
/// A relationship type is a reusable entity that defines the structure of observable properties
/// and attributes of a relationship as well as its supported interfaces.
#[derive(Clone, Debug, Default, Depict, Resolve)]
#[depict(tag = tag::source_and_span)]
#[resolve(annotated_parameter=AnnotatedT)]
pub struct RelationshipType<AnnotatedT>
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

    ///	An optional map of property definitions for the relationship type.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub properties: PropertyDefinitions<AnnotatedT>,

    /// An optional map of attribute definitions for the relationship type.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub attributes: AttributeDefinitions<AnnotatedT>,

    /// An optional map of interface definitions supported by the relationship type.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub interfaces: InterfaceDefinitions<AnnotatedT>,

    /// An optional list of one or more names of capability types that are valid targets
    /// for this relationship. If undefined, all capability types are valid.
    #[resolve]
    #[depict(option, iter(item), as(depict))]
    pub valid_capability_types: Option<Vec<FullName>>,

    /// An optional list of one or more names of node types that are valid targets for
    /// this relationship. If undefined, all node types are valid targets.
    #[resolve]
    #[depict(option, iter(item), as(depict))]
    pub valid_target_node_types: Option<Vec<FullName>>,

    /// An optional list of one or more names of node types that are valid sources for
    /// this relationship. If undefined, all node types are valid sources.
    #[resolve]
    #[depict(option, iter(item), as(depict))]
    pub valid_source_node_types: Option<Vec<FullName>>,

    #[resolve(annotations)]
    #[depict(skip)]
    pub(crate) annotations: StructAnnotations,

    #[depict(skip)]
    completion_state: CompletionState,
}

impl_type_entity!(RelationshipType);

impl<AnnotatedT> Entity for RelationshipType<AnnotatedT>
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
            RELATIONSHIP_TYPE,
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
        complete_subentity_map_field!(
            interface,
            interfaces,
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
                errors, self, "valid_capability_types",
                complete_type_list(
                    &mut self.valid_capability_types,
                    &parent.valid_capability_types,
                    catalog,
                    source_id,
                    errors,
                )?;
            );

            errors_with_fallback_annotations_from_field!(
                errors, self, "valid_target_node_types",
                complete_type_list(
                    &mut self.valid_target_node_types,
                    &parent.valid_target_node_types,
                    catalog,
                    source_id,
                    errors,
                )?;
            );

            errors_with_fallback_annotations_from_field!(
                errors, self, "valid_source_node_types",
                complete_type_list(
                    &mut self.valid_source_node_types,
                    &parent.valid_source_node_types,
                    catalog,
                    source_id,
                    errors,
                )?;
            );
        }

        self.completion_state = CompletionState::Complete;
        Ok(())
    }
}

//
// RelationshipTypes
//

/// Map of [RelationshipType].
pub type RelationshipTypes<AnnotatedT> = BTreeMap<Name, RelationshipType<AnnotatedT>>;
