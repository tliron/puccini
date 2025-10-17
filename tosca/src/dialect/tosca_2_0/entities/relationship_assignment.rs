use super::{
    super::super::super::grammar::*, interface_assignment::*, relationship_definition::*, value_assignment::*,
};

use {
    compris::{annotate::*, resolve::*},
    depiction::*,
    kutil::std::immutable::*,
};

//
// RelationshipAssignment
//

/// The relationship keyname in a requirement assignment typically specifies a relationship
/// assignment that provides information needed by TOSCA Orchestrators to construct a relationship
/// to the TOSCA node that is the target of the requirement.
///
/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
#[derive(Clone, Debug, Default, Depict, Resolve)]
#[depict(tag = tag::source_and_span)]
#[resolve(annotated_parameter=AnnotatedT)]
pub struct RelationshipAssignment<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    /// The optional keyname used to provide the name of the relationship type for the requirement
    /// assignment's relationship.
    #[resolve(key = "type")]
    #[depict(as(depict))]
    pub type_name: FullName,

    /// An optional map of property assignments for the relationship.
    #[resolve]
    #[depict(iter(kv), as(depict), key_as(display), key_style(name))]
    pub properties: ValueAssignments<AnnotatedT>,

    /// An optional map of attribute assignments for the relationship.
    #[resolve]
    #[depict(iter(kv), as(depict), key_as(display), key_style(name))]
    pub attributes: ValueAssignments<AnnotatedT>,

    /// An optional map of interface assignments for the corresponding interface definitions in the
    /// relationship type.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub interfaces: InterfaceAssignments<AnnotatedT>,

    #[resolve(annotations)]
    #[depict(skip)]
    pub(crate) annotations: StructAnnotations,
}

impl<AnnotatedT> Subentity<RelationshipDefinition<AnnotatedT>> for RelationshipAssignment<AnnotatedT>
where
    AnnotatedT: 'static + Annotated + Clone + Default,
{
    fn complete(
        &mut self,
        _name: Option<ByteString>,
        relationship_definition: Option<&RelationshipDefinition<AnnotatedT>>,
        relationship_definition_namespace: Option<&Namespace>,
        context: &mut CompletionContext,
    ) -> Result<(), ToscaError<WithAnnotations>> {
        complete_name_field!(type_name, self, relationship_definition, relationship_definition_namespace, context);

        complete_subentity_map_field!(
            property,
            properties,
            self,
            relationship_definition,
            relationship_definition_namespace,
            true,
            context
        );
        complete_subentity_map_field!(
            attribute,
            attributes,
            self,
            relationship_definition,
            relationship_definition_namespace,
            true,
            context
        );
        complete_subentity_map_field!(
            interface,
            interfaces,
            self,
            relationship_definition,
            relationship_definition_namespace,
            true,
            context
        );

        Ok(())
    }
}

impl<AnnotatedT> ToNamespace<RelationshipAssignment<AnnotatedT>> for RelationshipDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn to_namespace(&self, namespace: Option<&Namespace>) -> RelationshipAssignment<AnnotatedT> {
        RelationshipAssignment {
            type_name: self.type_name.to_namespace(namespace),
            properties: self.properties.to_namespace(namespace),
            attributes: self.attributes.to_namespace(namespace),
            interfaces: self.interfaces.to_namespace(namespace),
            annotations: self.annotations.clone_fields(&["type_name", "properties", "attributes", "interfaces"]),
        }
    }
}
