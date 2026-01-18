use super::{
    super::{super::super::grammar::*, dialect::*},
    attribute_definition::*,
    interface_definition::*,
    property_definition::*,
    relationship_type::*,
};

use {
    compris::{annotate::*, depict::*, resolve::*},
    depiction::*,
    kutil::std::immutable::*,
    problemo::*,
};

//
// RelationshipDefinition
//

/// The relationship keyname in a requirement definition specifies a relationship definition that
/// provides information needed by TOSCA Orchestrators to construct a relationship to the TOSCA
/// node that contains the matching target capability.
///
/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
#[derive(Clone, Debug, Default, Depict, Resolve)]
#[depict(tag = tag::source_and_span)]
#[resolve(annotated_parameter=AnnotatedT)]
pub struct RelationshipDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    /// The mandatory keyname used to provide the name of the relationship type used for the
    /// relationship.
    ///
    /// Puccini note: *Not* mandatory, as it can be inherited from parent.
    #[resolve(single, key = "type")]
    #[depict(as(depict))]
    pub type_name: FullName,

    /// The optional description of the relationship definition.
    #[resolve]
    #[depict(option, style(string))]
    pub description: Option<ByteString>,

    /// Defines a section used to declare additional information.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub metadata: Metadata<AnnotatedT>,

    ///	An optional map of property refinements for the relationship definition. The referred
    /// properties must have been defined in the relationship type definition referred by the type
    /// keyname. New properties may not be added.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub properties: PropertyDefinitions<AnnotatedT>,

    /// An optional map of attribute refinements for the relationship definition. The referred
    /// attributes must have been defined in the relationship type definition referred by the
    /// type keyname. New attributes may not be added.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub attributes: AttributeDefinitions<AnnotatedT>,

    /// The optional keyname used to define interface refinements for interfaces defined by the
    /// relationship type.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub interfaces: InterfaceDefinitions<AnnotatedT>,

    #[resolve(annotations)]
    #[depict(skip)]
    pub(crate) annotations: StructAnnotations,
}

impl<AnnotatedT> Subentity<Self> for RelationshipDefinition<AnnotatedT>
where
    AnnotatedT: 'static + Annotated + Clone + Default,
{
    fn complete(
        &mut self,
        _name: Option<&Name>,
        parent: Option<&Self>,
        parent_namespace: Option<&Namespace>,
        context: &mut CompletionContext,
    ) -> Result<(), Problem> {
        complete_type_name_field!(self, parent, parent_namespace, true, context);
        complete_subentity_map_field!(property, properties, self, parent, parent_namespace, true, context);
        complete_subentity_map_field!(attribute, attributes, self, parent, parent_namespace, true, context);
        complete_subentity_map_field!(interface, interfaces, self, parent, parent_namespace, true, context);

        let (relationship_type, relationship_type_namespace) =
            completed_entity_from_full_name_field!(RELATIONSHIP_TYPE, RelationshipType, self, type_name, context);

        complete_subentity_map_field!(
            property,
            properties,
            self,
            relationship_type,
            relationship_type_namespace,
            true,
            context
        );
        complete_subentity_map_field!(
            attribute,
            attributes,
            self,
            relationship_type,
            relationship_type_namespace,
            true,
            context
        );
        complete_subentity_map_field!(
            interface,
            interfaces,
            self,
            relationship_type,
            relationship_type_namespace,
            true,
            context
        );

        Ok(())
    }
}

impl<AnnotatedT> ToNamespace<Self> for RelationshipDefinition<AnnotatedT>
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
            interfaces: self.interfaces.to_namespace(namespace),
            annotations: self.annotations.clone(),
        }
    }
}
