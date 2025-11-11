use super::{
    super::{super::super::grammar::*, data::*, dialect::*},
    attribute_definition::*,
    property_definition::*,
};

use {
    compris::{annotate::*, resolve::*},
    depiction::*,
    kutil::std::immutable::*,
    std::collections::*,
};

//
// CapabilityType
//

/// A capability type is a reusable entity that describes the properties and attributes of a
/// capability that a node type can declare to expose. Requirements that are declared as part of
/// one node can be fulfilled by the capabilities declared by another node.
///
/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
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
        derivation_path: &mut DerivationPath,
        context: &mut CompletionContext,
    ) -> Result<(), ToscaError<WithAnnotations>> {
        assert!(self.completion_state == CompletionState::Incomplete);
        self.completion_state = CompletionState::Cannot;

        let (parent, parent_namespace) = completed_entity_checked_from_full_name_field!(
            CAPABILITY_TYPE,
            self,
            derived_from,
            derivation_path,
            context
        );

        complete_subentity_map_field!(property, properties, self, parent, parent_namespace, true, context);
        complete_subentity_map_field!(attribute, attributes, self, parent, parent_namespace, true, context);
        complete_type_list_field!(valid_source_node_types, self, parent, context);
        complete_type_list_field!(valid_relationship_types, self, parent, context);

        self.completion_state = CompletionState::Complete;
        Ok(())
    }
}

impl<AnnotatedT> ToNamespace<Self> for CapabilityType<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn to_namespace(&self, namespace: Option<&Namespace>) -> Self {
        Self {
            derived_from: self.derived_from.to_namespace(namespace),
            version: self.version.clone(),
            metadata: self.metadata.clone(),
            description: self.description.clone(),
            properties: self.properties.to_namespace(namespace),
            attributes: self.attributes.to_namespace(namespace),
            valid_source_node_types: self.valid_source_node_types.to_namespace(namespace),
            valid_relationship_types: self.valid_relationship_types.to_namespace(namespace),
            annotations: self.annotations.clone(),
            completion_state: self.completion_state,
        }
    }
}

//
// CapabilityTypes
//

/// Map of [CapabilityType].
pub type CapabilityTypes<AnnotatedT> = BTreeMap<Name, CapabilityType<AnnotatedT>>;
