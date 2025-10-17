use super::{
    super::{super::super::grammar::*, dialect::*},
    attribute_definition::*,
    capability_type::*,
    property_definition::*,
};

use {
    compris::{annotate::*, resolve::*},
    depiction::*,
    kutil::std::immutable::*,
    std::collections::*,
};

//
// CapabilityDefinition
//

/// A capability definition defines a typed set of data that a node can expose and that is used to
/// describe a relevant feature of the component described by the node that can be used to fulfill
/// a requirement exposed by another node. A capability is defined as part of a node type
/// definition and may be refined during node type derivation.
///
/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
#[derive(Clone, Debug, Default, Depict, Resolve)]
#[depict(tag = tag::source_and_span)]
#[resolve(annotated_parameter=AnnotatedT)]
pub struct CapabilityDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    /// The mandatory name of the capability type on which this capability definition is based.
    #[resolve(single, key = "type")]
    #[depict(as(depict))]
    pub type_name: FullName,

    /// The optional description of the Capability definition.
    #[resolve]
    #[depict(option, style(string))]
    pub description: Option<ByteString>,

    /// Defines a section used to declare additional information.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub metadata: Metadata<AnnotatedT>,

    /// An optional map of property refinements for the capability definition. The referred
    /// properties must have been defined in the capability type definition referred to by the
    /// type keyname. New properties may not be added.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub properties: PropertyDefinitions<AnnotatedT>,

    /// An optional map of attribute refinements for the capability definition. The referred
    /// attributes must have been defined in the capability type definition referred by the type
    /// keyname. New attributes may not be added.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub attributes: AttributeDefinitions<AnnotatedT>,

    /// An optional list of one or more valid names of node types that are supported as valid
    /// sources of any relationship established to the declared capability type. If undefined, all
    /// node types are valid sources. If valid_source_node_types is defined in the capability type,
    /// each element in this list must either be or derived from an element in the list defined in
    /// the type.
    #[resolve]
    #[depict(option, iter(item), as(depict))]
    pub valid_source_node_types: Option<Vec<FullName>>,

    /// An optional list of one or more valid names of relationship types that are supported as
    /// valid types of any relationship established to the declared capability type. If undefined,
    /// all relationship types are valid. If valid_relationship_types is defined in the capability
    /// type, each element in this list must either be or derived from an element in the list
    /// defined in the type.
    #[resolve]
    #[depict(option, iter(item), as(depict))]
    pub valid_relationship_types: Option<Vec<FullName>>,

    #[resolve(annotations)]
    #[depict(skip)]
    pub(crate) annotations: StructAnnotations,
}

impl<AnnotatedT> Subentity<Self> for CapabilityDefinition<AnnotatedT>
where
    AnnotatedT: 'static + Annotated + Clone + Default,
{
    fn complete(
        &mut self,
        _name: Option<ByteString>,
        parent: Option<&Self>,
        parent_namespace: Option<&Namespace>,
        context: &mut CompletionContext,
    ) -> Result<(), ToscaError<WithAnnotations>> {
        complete_name_field!(type_name, self, parent, parent_namespace, context);
        complete_subentity_map_field!(property, properties, self, parent, parent_namespace, false, context);
        complete_subentity_map_field!(attribute, attributes, self, parent, parent_namespace, false, context);
        complete_type_list_field!(valid_source_node_types, self, parent, context);
        complete_type_list_field!(valid_relationship_types, self, parent, context);

        let (capability_type, capability_type_namespace) =
            entity_from_full_name_field!(CAPABILITY_TYPE, CapabilityType, self, type_name, context);

        complete_subentity_map_field!(
            property,
            properties,
            self,
            capability_type,
            capability_type_namespace,
            true,
            context
        );
        complete_subentity_map_field!(
            attribute,
            attributes,
            self,
            capability_type,
            capability_type_namespace,
            true,
            context
        );
        complete_type_list_field!(valid_source_node_types, self, capability_type, context);
        complete_type_list_field!(valid_relationship_types, self, capability_type, context);

        Ok(())
    }
}

impl<AnnotatedT> ToNamespace<Self> for CapabilityDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn to_namespace(&self, namespace: Option<&Namespace>) -> Self {
        Self {
            type_name: self.type_name.to_namespace(namespace),
            description: self.description.clone(),
            metadata: self.metadata.clone(),
            properties: self.properties.to_namespace(namespace),
            attributes: self.attributes.to_namespace(namespace),
            valid_source_node_types: self.valid_source_node_types.to_namespace(namespace),
            valid_relationship_types: self.valid_relationship_types.to_namespace(namespace),
            annotations: self.annotations.clone(),
        }
    }
}

//
// CapabilityDefinitions
//

/// Map of [CapabilityDefinition].
pub type CapabilityDefinitions<AnnotatedT> = BTreeMap<ByteString, CapabilityDefinition<AnnotatedT>>;
