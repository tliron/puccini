use super::{super::super::super::grammar::*, implementation_definition::*, parameter_definition::*};

use {
    compris::{annotate::*, resolve::*},
    depiction::*,
    kutil::std::immutable::*,
    std::collections::*,
};

//
// OperationDefinition
//

/// An operation definition defines a function or procedure to which an operation implementation
/// can be bound.
///
/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
#[derive(Clone, Debug, Default, Depict, Resolve)]
#[depict(tag = tag::source_and_span)]
#[resolve(annotated_parameter=AnnotatedT)]
pub struct OperationDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    /// The optional description string for the associated operation.
    #[resolve]
    #[depict(option, style(string))]
    pub description: Option<ByteString>,

    /// The optional definition of the operation implementation. May not be used in an interface
    /// type definition (i.e. where an operation is initially defined), but only during refinements.
    #[resolve(single)]
    #[depict(option, as(depict))]
    pub implementation: Option<ImplementationDefinition<AnnotatedT>>,

    /// The optional map of parameter definitions for operation input values.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub inputs: ParameterDefinitions<AnnotatedT>,

    /// The optional map of parameter definitions for operation output values. Only as part of
    /// node and relationship type definitions, the output definitions may include mappings onto
    /// attributes of the node or relationship type that contains the definition.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub outputs: ParameterDefinitions<AnnotatedT>,

    #[resolve(annotations)]
    #[depict(skip)]
    pub(crate) annotations: StructAnnotations,
}

impl<AnnotatedT> Subentity<Self> for OperationDefinition<AnnotatedT>
where
    AnnotatedT: 'static + Annotated + Clone + Default,
{
    fn complete(
        &mut self,
        _name: Option<&Name>,
        parent: Option<&Self>,
        parent_namespace: Option<&Namespace>,
        context: &mut CompletionContext,
    ) -> Result<(), ToscaError<WithAnnotations>> {
        complete_subentity_map_field!(input, inputs, self, parent, parent_namespace, true, context);
        complete_subentity_map_field!(output, outputs, self, parent, parent_namespace, true, context);
        complete_subentity_field!(implementation, self, parent, parent_namespace, context);
        Ok(())
    }
}

impl<AnnotatedT> ToNamespace<Self> for OperationDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn to_namespace(&self, namespace: Option<&Namespace>) -> Self {
        Self {
            description: self.description.clone(),
            implementation: self.implementation.to_namespace(namespace),
            inputs: self.inputs.to_namespace(namespace),
            outputs: self.outputs.to_namespace(namespace),
            annotations: self.annotations.clone(),
        }
    }
}

//
// OperationDefinitions
//

/// Map of [OperationDefinition].
pub type OperationDefinitions<AnnotatedT> = BTreeMap<Name, OperationDefinition<AnnotatedT>>;
