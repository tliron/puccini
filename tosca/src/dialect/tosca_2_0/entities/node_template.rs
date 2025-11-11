use super::{
    super::{super::super::grammar::*, dialect::*},
    artifact_assignment::*,
    capability_assignment::*,
    interface_assignment::*,
    node_type::*,
    requirement_assignment::*,
    value_assignment::*,
};

use {
    compris::{annotate::*, normal::*, resolve::*},
    depiction::*,
    kutil::std::{error::*, immutable::*},
    std::collections::*,
};

//
// NodeTemplate
//

/// A node template specifies the occurrence of one or more instances of a component of a given type
/// in an application or service. A node template defines application-specific values for the
/// properties, relationships, or interfaces defined by its node type.
///
/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
#[derive(Clone, Debug, Default, Depict, Resolve)]
#[depict(tag = tag::source_and_span)]
#[resolve(annotated_parameter=AnnotatedT)]
pub struct NodeTemplate<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    /// The mandatory name of the node type on which the node template is based.
    ///
    /// Puccini note: *Not* mandatory, as it can be copied via "copy".
    #[resolve(key = "type")]
    #[depict(option, as(display), style(name))]
    pub type_name: Option<FullName>,

    /// An optional description for the node template.
    #[resolve]
    #[depict(option, style(string))]
    pub description: Option<ByteString>,

    /// Defines a section used to declare additional information.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub metadata: Metadata<AnnotatedT>,

    /// An optional list of directive values to provide processing instructions to orchestrators
    /// and tooling.
    #[resolve]
    #[depict(iter(item), style(symbol))]
    pub directives: Vec<ByteString>,

    /// An optional map of property value assignments for the node template.
    #[resolve]
    #[depict(iter(kv), as(depict), key_as(display), key_style(name))]
    pub properties: ValueAssignments<AnnotatedT>,

    /// An optional map of attribute value assignments for the node template.
    #[resolve]
    #[depict(iter(kv), as(depict), key_as(display), key_style(name))]
    pub attributes: ValueAssignments<AnnotatedT>,

    /// An optional map of requirement assignments for the node template.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub requirements: RequirementAssignments<AnnotatedT>,

    /// An optional map of capability assignments for the node template.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub capabilities: CapabilityAssignments<AnnotatedT>,

    /// An optional map of interface assignments for the node template.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub interfaces: InterfaceAssignments<AnnotatedT>,

    /// An optional map of artifact definitions for the node template.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub artifacts: ArtifactAssignments<AnnotatedT>,

    /// An optional keyname that specifies how many node representations must be created from
    /// this node template. If not defined, the assumed count value is 1.
    #[resolve]
    #[depict(option, style(number))]
    pub count: Option<u64>,

    /// The optional filter definition that TOSCA orchestrators will use to select an already
    /// existing node if this node template is marked with the "select" directive.
    #[resolve]
    #[depict(option, as(depict))]
    pub node_filter: Option<Variant<AnnotatedT>>,

    /// The optional (symbolic) name of another node template from which to copy all keynames and
    /// values into this node template.
    #[resolve]
    #[depict(option, as(depict))]
    pub copy: Option<Name>,

    #[resolve(annotations)]
    #[depict(skip)]
    pub(crate) annotations: StructAnnotations,

    #[depict(skip)]
    completion_state: CompletionState,
}

impl<AnnotatedT> Entity for NodeTemplate<AnnotatedT>
where
    AnnotatedT: 'static + Annotated + Clone + Default,
{
    fn completion_state(&self) -> CompletionState {
        self.completion_state
    }

    fn complete(
        &mut self,
        _derivation_path: &mut DerivationPath,
        context: &mut CompletionContext,
    ) -> Result<(), ToscaError<WithAnnotations>> {
        assert!(self.completion_state == CompletionState::Incomplete);
        self.completion_state = CompletionState::Cannot;

        if let Some(copy) = completed_entity_from_optional_name_field!(NODE_TEMPLATE, NodeTemplate, self, copy, context)
        {
            complete_optional_field!(type_name, self, copy);
            complete_optional_field!(description, self, copy);
            complete_field!(metadata, self, copy);
            complete_field!(directives, self, copy);
            complete_field!(properties, self, copy);
            complete_field!(attributes, self, copy);
            complete_field!(requirements, self, copy);
            complete_field!(capabilities, self, copy);
            complete_field!(interfaces, self, copy);
            complete_field!(artifacts, self, copy);
            complete_optional_field!(count, self, copy);
            complete_optional_field!(node_filter, self, copy);
        }

        if self.type_name.is_none() {
            context.errors.give(MissingRequiredKeyError::new("type".into()).with_annotations_from(self))?;
            return Ok(());
        }

        let (node_type, node_type_namespace) =
            completed_entity_from_optional_full_name_field!(NODE_TYPE, NodeType, self, type_name, context);

        complete_subentity_map_field!(property, properties, self, node_type, node_type_namespace, true, context);
        complete_subentity_map_field!(attribute, attributes, self, node_type, node_type_namespace, true, context);
        complete_subentity_taxonomy_field!(
            requirement,
            requirements,
            self,
            node_type,
            node_type_namespace,
            true,
            context
        );
        complete_subentity_map_field!(capability, capabilities, self, node_type, node_type_namespace, true, context);
        complete_subentity_map_field!(interface, interfaces, self, node_type, node_type_namespace, true, context);
        complete_subentity_map_field!(artifact, artifacts, self, node_type, node_type_namespace, true, context);

        self.completion_state = CompletionState::Complete;
        Ok(())
    }
}

//
// NodeTemplates
//

/// Map of [NodeTemplate].
pub type NodeTemplates<AnnotatedT> = BTreeMap<Name, NodeTemplate<AnnotatedT>>;
