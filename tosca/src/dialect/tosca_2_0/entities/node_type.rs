use super::{
    super::{super::super::grammar::*, data::*, dialect::*},
    artifact_definition::*,
    attribute_definition::*,
    capability_definition::*,
    interface_definition::*,
    property_definition::*,
    requirement_definition::*,
};

use {
    compris::{annotate::*, resolve::*},
    depiction::*,
    kutil::std::immutable::*,
    std::collections::*,
};

//
// NodeType
//

/// A node type is a reusable entity that defines the structure of observable properties and
/// attributes of a node, the capabilities and requirements of that node, as well as its
/// supported interfaces and the artifacts it uses.
///
/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
#[derive(Clone, Debug, Default, Depict, Resolve)]
#[depict(tag = tag::source_and_span)]
#[resolve(annotated_parameter=AnnotatedT)]
pub struct NodeType<AnnotatedT>
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

    ///	An optional map of property definitions for the node type.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub properties: PropertyDefinitions<AnnotatedT>,

    /// An optional map of attribute definitions for the node type.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub attributes: AttributeDefinitions<AnnotatedT>,

    /// An optional map of capability definitions for the node type.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub capabilities: CapabilityDefinitions<AnnotatedT>,

    /// An optional list of requirement definitions for the node type.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub requirements: RequirementDefinitions<AnnotatedT>,

    /// An optional map of interface definitions supported by the node type.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub interfaces: InterfaceDefinitions<AnnotatedT>,

    /// An optional map of artifact definitions for the node type.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub artifacts: ArtifactDefinitions<AnnotatedT>,

    #[resolve(annotations)]
    #[depict(skip)]
    pub(crate) annotations: StructAnnotations,

    #[depict(skip)]
    completion_state: CompletionState,
}

impl_type_entity!(NodeType);

impl<AnnotatedT> Entity for NodeType<AnnotatedT>
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

        let (parent, parent_namespace) =
            completed_entity_checked_from_full_name_field!(NODE_TYPE, self, derived_from, derivation_path, context);

        complete_subentity_map_field!(property, properties, self, parent, parent_namespace, false, context);
        complete_subentity_map_field!(attribute, attributes, self, parent, parent_namespace, false, context);
        complete_subentity_map_field!(capability, capabilities, self, parent, parent_namespace, false, context);
        complete_subentity_taxonomy_field!(requirement, requirements, self, parent, parent_namespace, false, context);
        complete_subentity_map_field!(interface, interfaces, self, parent, parent_namespace, false, context);
        complete_subentity_map_field!(artifact, artifacts, self, parent, parent_namespace, false, context);

        self.completion_state = CompletionState::Complete;
        Ok(())
    }
}

impl<AnnotatedT> ToNamespace<Self> for NodeType<AnnotatedT>
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
            capabilities: self.capabilities.to_namespace(namespace),
            requirements: self.requirements.to_namespace(namespace),
            interfaces: self.interfaces.to_namespace(namespace),
            artifacts: self.artifacts.to_namespace(namespace),
            annotations: self.annotations.clone(),
            completion_state: self.completion_state,
        }
    }
}

//
// NodeTypes
//

/// Map of [NodeType].
pub type NodeTypes<AnnotatedT> = BTreeMap<Name, NodeType<AnnotatedT>>;
