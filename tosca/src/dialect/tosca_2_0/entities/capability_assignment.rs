use super::{super::super::super::grammar::*, capability_definition::*, value::*};

use {
    compris::{annotate::*, resolve::*},
    kutil::{
        cli::depict::*,
        std::{error::*, immutable::*},
    },
    std::collections::*,
};

//
// CapabilityAssignment
//

/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
///
/// A capability assignment allows node template authors to assign values to properties and
/// attributes for a capability definition that is part of the node template's type definition.
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
        _name: Option<ByteString>,
        capability_definition: Option<(&CapabilityDefinition<AnnotatedT>, &Scope)>,
        catalog: &mut Catalog,
        source_id: &SourceID,
        errors: ToscaErrorRecipientRef,
    ) -> Result<(), ToscaError<WithAnnotations>> {
        let errors = &mut errors.to_error_recipient();

        complete_map_field!("property", properties, self, capability_definition, catalog, source_id, errors);
        complete_map_field!("attribute", attributes, self, capability_definition, catalog, source_id, errors);

        Ok(())
    }
}

impl<AnnotatedT> ConvertIntoScope<CapabilityAssignment<AnnotatedT>> for CapabilityDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn convert_into_scope(&self, scope: &Scope) -> CapabilityAssignment<AnnotatedT> {
        CapabilityAssignment {
            properties: self.properties.convert_into_scope(scope),
            attributes: self.attributes.convert_into_scope(scope),
            annotations: clone_struct_annotations(&self.annotations, &["properties", "attributes"]),
            ..Default::default()
        }
    }
}

//
// CapabilityAssignments
//

/// Map of [CapabilityAssignment].
pub type CapabilityAssignments<AnnotatedT> = BTreeMap<ByteString, CapabilityAssignment<AnnotatedT>>;
