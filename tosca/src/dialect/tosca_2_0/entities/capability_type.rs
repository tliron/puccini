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

//
// CapabilityType
//

/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
///
/// A capability type is a reusable entity that describes the properties and attributes of a
/// capability that a node type can declare to expose. Requirements that are declared as part of
/// one node can be fulfilled by the capabilities declared by another node.
#[derive(Clone, Debug, Default, Depict, Resolve)]
#[depict(tag = tag::source_and_span)]
#[resolve(annotated_parameter=AnnotatedT)]
pub struct CapabilityType<AnnotatedT>
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

    /// An optional map of property definitions for the capability type.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub properties: PropertyDefinitions<AnnotatedT>,

    /// An optional map of attribute definitions for the capability type.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub attributes: AttributeDefinitions<AnnotatedT>,

    /// An optional list of one or more valid names of node types that are supported as
    /// valid sources of any relationship established to the declared capability type. If
    /// undefined, all node types are valid sources.
    #[resolve]
    #[depict(option, iter(item), as(depict))]
    pub valid_source_node_types: Option<Vec<FullName>>,

    /// An optional list of one or more valid names of relationship types that are supported
    /// as valid types of any relationship established to the declared capability type. If
    /// undefined, all relationship types are valid.
    #[resolve]
    #[depict(option, iter(item), as(depict))]
    pub valid_relationship_types: Option<Vec<FullName>>,

    #[resolve(annotations)]
    #[depict(skip)]
    pub(crate) annotations: StructAnnotations,

    #[depict(skip)]
    completion_state: CompletionState,
}

impl_type_entity!(CapabilityType);

impl<AnnotatedT> Entity for CapabilityType<AnnotatedT>
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
            CAPABILITY_TYPE,
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
                errors, self, "valid_source_node_types",
                complete_type_list(
                    &mut self.valid_source_node_types,
                    &parent.valid_source_node_types,
                    catalog,
                    source_id,
                    errors,
                )?;
            );

            errors_with_fallback_annotations_from_field!(
                errors, self, "valid_relationship_types",
                complete_type_list(
                    &mut self.valid_relationship_types,
                    &parent.valid_relationship_types,
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
// CapabilityTypes
//

/// Map of [CapabilityType].
pub type CapabilityTypes<AnnotatedT> = BTreeMap<Name, CapabilityType<AnnotatedT>>;
