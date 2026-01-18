use super::{super::super::super::grammar::*, relationship_assignment::*, requirement_definition::*};

use {
    compris::{annotate::*, depict::*, normal::*, resolve::*},
    depiction::*,
    kutil::std::immutable::*,
    problemo::*,
    smart_default::*,
};

//
// RequirementAssignment
//

/// A requirement assignment is used by node template authors to provide assignments for the
/// corresponding requirement definition in the node template's node type. This includes specifying
/// target nodes, either by providing symbolic names of target nodes or by providing selection
/// criteria for TOSCA orchestrators to find candidate nodes that can be used to fulfill the
/// requirement. In addition, requirement assignments must uniquely identify the specific target
/// capability in the target node for the requirement. Requirement assignments must also assign
/// values to properties and attributes defined in the relationship definition that is part of the
/// requirement definition, and provide values for the input parameters defined by the relationship
/// definition's interfaces.
///
/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
#[derive(Clone, Debug, Depict, Resolve, SmartDefault)]
#[depict(tag = tag::source_and_span)]
#[resolve(annotated_parameter=AnnotatedT)]
pub struct RequirementAssignment<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    /// The optional keyname used to identify the target node of the requirement:
    /// - This can either be the symbolic name of a node template, where the TOSCA processor will
    ///   select a node representation created from that template. If the count of the node
    ///   template is 1 then the potential target is unique, otherwise the processor can select
    ///   from several node representations.
    /// - It can also be a 2-entry list, where the first entry is a string denoting the symbolic
    ///   name of a node template, while the second entry is an index, thus uniquely identifying
    ///   the node representation when multiple representations are created from the same node
    ///   template. The index is a non-negative integer, with 0 being the first index. Note that
    ///   functions like $node_index or $relationship_index may be used to match the target index
    ///   with the source/relationship index. More information on multiplicity and node and
    ///   relationship indexes can be found in Chapter 14.
    /// - Finally, it can also be the name of a node type that the TOSCA processor will use to
    ///   select a type-compatible target node to fulfill the requirement.
    #[resolve(single)]
    #[depict(option, as(depict))]
    pub node: Option<IndexedFullName>,

    /// The optional keyname used to identify the target capability of the requirement. This can
    /// either be the name of a capability defined within a target node or the name of a target
    /// capability type that the TOSCA orchestrator will use to select a type-compatible target
    /// node to fulfill the requirement at runtime.
    #[resolve]
    #[depict(option, as(depict))]
    pub capability: Option<FullName>,

    /// The conditional keyname used to provide values for the relationship definition in the
    /// corresponding requirement definition. This keyname can also be overloaded to define a
    /// symbolic name that references a relationship template defined elsewhere in the service
    /// template.
    #[resolve]
    #[depict(option, as(depict))]
    pub relationship: Option<RelationshipAssignment<AnnotatedT>>,

    /// The optional keyname that allows the inclusion of an allocation block. The allocation
    /// block contains a map of property assignments that semantically represent allocations from
    /// the property with the same name in the target capability. The allocation acts as a capacity
    /// filter for the target capability in the target node. When the requirement is resolved, a
    /// capability in a node is a valid target for the requirement relationship if for each
    /// property of the target capability, the sum of all existing allocations plus the current
    /// allocation is less than or equal to the property value.
    #[resolve]
    #[depict(option, as(depict))]
    pub allocation: Option<Variant<AnnotatedT>>,

    /// An optional keyname that sets the cardinality of the requirement assignment, that is how
    /// many relationships must be established from this requirement assignment. If not defined,
    /// the default count for an assignment is 1. Note that there can be multiple requirement
    /// assignments for a requirement with a specific symbolic name. The sum of all count values
    /// of assignments for a requirement with a specific symbolic name must be within the
    /// count_range defined in the requirement definition. Moreover, the sum of all count values
    /// of non-optional assignments for a requirement with a specific symbolic name must also be
    /// within the count_range defined in the requirement definition.
    #[default(1)]
    #[resolve]
    #[depict(style(number))]
    pub count: u64,

    /// The optional filter definition that TOSCA orchestrators will use to select a
    /// type-compatible target node that can fulfill the requirement at runtime.
    #[resolve]
    #[depict(option, as(depict))]
    pub node_filter: Option<Variant<AnnotatedT>>,

    /// An optional list of directive values to provide processing instructions to
    /// orchestrators and tooling.
    #[resolve]
    #[depict(iter(item), style(symbol))]
    pub directives: Vec<ByteString>,

    /// Describes if the fulfillment of this requirement assignment is optional (true) or not
    /// (false). If not specified, the requirement assignment must be fulfilled, i.e. the default
    /// value is false. Note also, that non-optional requirements have precedence, thus during a
    /// service deployment, the optional requirements for all nodes should be resolved only after
    /// the non-optional requirements for all nodes have been resolved.
    #[resolve]
    #[depict(style(symbol))]
    pub optional: bool,

    /// Description.
    #[depict(skip)]
    pub description: Option<ByteString>,

    /// Metadata.
    #[depict(skip)]
    pub metadata: Metadata<AnnotatedT>,

    #[resolve(annotations)]
    #[depict(skip)]
    pub(crate) annotations: StructAnnotations,
}

impl<AnnotatedT> Subentity<RequirementDefinition<AnnotatedT>> for RequirementAssignment<AnnotatedT>
where
    AnnotatedT: 'static + Annotated + Clone + Default,
{
    fn complete(
        &mut self,
        name: Option<&Name>,
        requirement_definition: Option<&RequirementDefinition<AnnotatedT>>,
        requirement_definition_namespace: Option<&Namespace>,
        context: &mut CompletionContext,
    ) -> Result<(), Problem> {
        // TODO: validate node (template) adheres to parent's node (type)

        // TODO: validate capability adheres to parent's capability type

        if let Some(requirement_definition) = requirement_definition {
            complete_optional_field_to!(relationship, self, requirement_definition, || Some(
                requirement_definition.relationship.to_namespace(requirement_definition_namespace)
            ));
        }

        if let Some(relationship) = &mut self.relationship {
            relationship.complete(
                name,
                requirement_definition.map(|parent| &parent.relationship),
                requirement_definition_namespace,
                context,
            )?;
        }

        Ok(())
    }
}

impl<AnnotatedT> ToNamespace<RequirementAssignment<AnnotatedT>> for RequirementDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn to_namespace(&self, namespace: Option<&Namespace>) -> RequirementAssignment<AnnotatedT> {
        RequirementAssignment {
            node: self.node.clone().map(|full_name| IndexedFullName::new(full_name, None)),
            capability: Some(self.capability.clone()),
            relationship: Some(self.relationship.to_namespace(namespace)),
            node_filter: self.node_filter.clone(),
            description: self.description.clone(),
            metadata: self.metadata.clone(),
            annotations: self.annotations.clone_fields(&["node", "capability", "relationship", "node_filter"]),
            ..Default::default()
        }
    }
}

// TODO: unused

//
// RequirementAssignmentCapability
//

/// Requirement assignment capability.
pub enum RequirementAssignmentCapability {
    /// Capability type name.
    TypeName(FullName),

    /// Capability name.
    Name(Name),
}

//
// RequirementAssignmentNode
//

/// Requirement assignment node.
pub enum RequirementAssignmentNode {
    /// Node type name.
    TypeName(FullName),

    /// Node template.
    Template((Name, usize)),
}

//
// RequirementAssignments
//

/// [Taxonomy] of [RequirementAssignment].
pub type RequirementAssignments<AnnotatedT> = Taxonomy<Name, RequirementAssignment<AnnotatedT>>;
