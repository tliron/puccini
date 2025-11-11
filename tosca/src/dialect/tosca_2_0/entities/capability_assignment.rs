use super::{super::super::super::grammar::*, capability_definition::*, value_assignment::*};

use {
    compris::{annotate::*, resolve::*},
    depiction::*,
    kutil::std::immutable::*,
    std::collections::*,
};

//
// CapabilityAssignment
//

/// A capability assignment allows node template authors to assign values to properties and
/// attributes for a capability definition that is part of the node template's type definition.
///
/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
#[derive(Clone, Debug, Default, Depict, Resolve)]
#[depict(tag = tag::source_and_span)]
#[resolve(annotated_parameter=AnnotatedT)]
pub struct CapabilityAssignment<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    /// Map of property assignments.
    #[resolve]
    #[depict(iter(kv), as(depict), key_as(display), key_style(name))]
    pub properties: ValueAssignments<AnnotatedT>,

    /// Map of attribute assignments.
    #[resolve]
    #[depict(iter(kv), as(depict), key_as(display), key_style(name))]
    pub attributes: ValueAssignments<AnnotatedT>,

    /// An optional list of directive values to provide processing instructions to orchestrators
    /// and tooling.
    #[resolve]
    #[depict(iter(item), style(symbol))]
    pub directives: Vec<ByteString>,

    /// Type name.
    #[depict(skip)]
    pub type_name: FullName,

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

impl<AnnotatedT> Subentity<CapabilityDefinition<AnnotatedT>> for CapabilityAssignment<AnnotatedT>
where
    AnnotatedT: 'static + Annotated + Clone + Default,
{
    fn complete(
        &mut self,
        _name: Option<&Name>,
        capability_definition: Option<&CapabilityDefinition<AnnotatedT>>,
        capability_definition_namespace: Option<&Namespace>,
        context: &mut CompletionContext,
    ) -> Result<(), ToscaError<WithAnnotations>> {
        complete_type_name_field!(self, capability_definition, capability_definition_namespace, false, context);
        complete_subentity_map_field!(
            property,
            properties,
            self,
            capability_definition,
            capability_definition_namespace,
            true,
            context
        );
        complete_subentity_map_field!(
            attribute,
            attributes,
            self,
            capability_definition,
            capability_definition_namespace,
            true,
            context
        );
        Ok(())
    }
}

impl<AnnotatedT> ToNamespace<CapabilityAssignment<AnnotatedT>> for CapabilityDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn to_namespace(&self, namespace: Option<&Namespace>) -> CapabilityAssignment<AnnotatedT> {
        CapabilityAssignment {
            properties: self.properties.to_namespace(namespace),
            attributes: self.attributes.to_namespace(namespace),
            type_name: self.type_name.to_namespace(namespace),
            description: self.description.clone(),
            metadata: self.metadata.clone(),
            annotations: self.annotations.clone_fields(&["properties", "attributes", "type_name"]),
            ..Default::default()
        }
    }
}

//
// CapabilityAssignments
//

/// Map of [CapabilityAssignment].
pub type CapabilityAssignments<AnnotatedT> = BTreeMap<Name, CapabilityAssignment<AnnotatedT>>;
