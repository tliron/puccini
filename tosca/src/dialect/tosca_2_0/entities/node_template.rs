use super::{
    super::{super::super::grammar::*, dialect::*},
    artifact_assignment::*,
    capability_assignment::*,
    interface_assignment::*,
    node_type::*,
    requirement_assignment::*,
    value::*,
};

use {
    compris::{annotate::*, normal::*, resolve::*},
    kutil::{
        cli::depict::*,
        std::{error::*, immutable::*},
    },
    std::collections::*,
};

//
// NodeTemplate
//

/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
///
/// A node template specifies the occurrence of one or more instances of a component of a given type
/// in an application or service. A node template defines application-specific values for the
/// properties, relationships, or interfaces defined by its node type.
#[derive(Clone, Debug, Default, Depict, Resolve)]
#[depict(tag = tag::source_and_span)]
#[resolve(annotated_parameter=AnnotatedT)]
pub struct NodeTemplate<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    /// The mandatory name of the node type on which the node template is based.
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
        catalog: &mut Catalog,
        source_id: &SourceID,
        _derivation_path: &mut DerivationPath,
        errors: ToscaErrorRecipientRef,
    ) -> Result<(), ToscaError<WithAnnotations>> {
        assert!(self.completion_state == CompletionState::Incomplete);
        self.completion_state = CompletionState::Cannot;

        let errors = &mut errors.to_error_recipient();

        if let Some(copy) = &self.copy {
            let Some(copy) = catalog.completed_entity::<NodeTemplate<AnnotatedT>, _, _>(
                NODE_TEMPLATE,
                &copy.clone().into(),
                source_id,
                errors,
            )?
            else {
                return Ok(());
            };

            complete_field_none!(type_name, self, copy);
            complete_field_none!(description, self, copy);
            complete_field_empty!(metadata, self, copy);
            complete_field_empty!(directives, self, copy);
            complete_field_empty!(properties, self, copy);
            complete_field_empty!(attributes, self, copy);
            complete_field_empty!(requirements, self, copy);
            complete_field_empty!(capabilities, self, copy);
            complete_field_empty!(interfaces, self, copy);
            complete_field_empty!(artifacts, self, copy);
            complete_field_none!(count, self, copy);
            complete_field_none!(node_filter, self, copy);
        }

        if self.type_name.is_none() {
            errors.give(MissingRequiredError::new("node type name".into(), Some("type_name".into())))?;
            return Ok(());
        }

        let (node_type, type_scope) =
            entity_from_name_field_option!(NODE_TYPE, NodeType, self, type_name, catalog, source_id, errors);

        complete_subentity_map_field!(
            property, properties, type_scope, self, node_type, true, catalog, source_id, errors
        );
        complete_subentity_map_field!(
            attribute, attributes, type_scope, self, node_type, true, catalog, source_id, errors
        );
        complete_subentity_tagged_values_field!(
            requirement,
            requirements,
            self,
            type_scope,
            node_type,
            true,
            catalog,
            source_id,
            errors
        );
        complete_subentity_map_field!(
            capability,
            capabilities,
            type_scope,
            self,
            node_type,
            true,
            catalog,
            source_id,
            errors
        );
        complete_subentity_map_field!(
            interface, interfaces, type_scope, self, node_type, true, catalog, source_id, errors
        );
        complete_subentity_map_field!(
            artifact, artifacts, type_scope, self, node_type, true, catalog, source_id, errors
        );

        self.completion_state = CompletionState::Complete;
        Ok(())
    }
}

//
// NodeTemplates
//

/// Map of [NodeTemplate].
pub type NodeTemplates<AnnotatedT> = BTreeMap<Name, NodeTemplate<AnnotatedT>>;
