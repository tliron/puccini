use super::{
    super::super::super::grammar::*, implementation_definition::*, operation_definition::*, value_assignment::*,
};

use {
    compris::{annotate::*, resolve::*},
    depiction::*,
    std::collections::*,
};

//
// OperationAssignment
//

/// An operation assignment may be used to assign values for input parameters, specify attribute
/// mappings for output parameters, and define/redefine the implementation definition of an already
/// defined operation in the interface definition. An operation assignment may be used inside
/// interface assignments inside node template or relationship template definitions (this includes
/// when operation assignments are part of a requirement assignment in a node template).
///
/// An operation assignment may add or change the implementation and description definition of the
/// operation. Assigning a value to an input parameter that had a fixed value specified during
/// operation definition or refinement is not allowed. Providing an attribute mapping for an output
/// parameter that was mapped during an operation refinement is also not allowed.
///
/// Note also that in the operation assignment we can use inputs and outputs that have not been
/// previously defined in the operation definition. This is equivalent to an ad-hoc definition of
/// a parameter, where the type is inferred from the assigned value (for input parameters) or from
/// the attribute to map to (for output parameters).
///
/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
#[derive(Clone, Debug, Default, Depict, Resolve)]
#[depict(tag = tag::source_and_span)]
#[resolve(annotated_parameter=AnnotatedT)]
pub struct OperationAssignment<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    /// The optional definition of the operation implementation. Overrides implementation provided
    /// at operation definition.
    #[resolve(single)]
    #[depict(option, as(depict))]
    pub implementation: Option<ImplementationDefinition<AnnotatedT>>,

    /// The optional map of parameter value assignments for assigning values to operation inputs.
    #[resolve]
    #[depict(iter(kv), as(depict), key_as(display), key_style(name))]
    pub inputs: ValueAssignments<AnnotatedT>,

    /// The optional map of parameter mapping assignments that specify how operation outputs are
    /// mapped onto attributes of the node or relationship that contains the operation definition.
    #[resolve]
    #[depict(iter(kv), as(depict), key_as(display), key_style(name))]
    pub outputs: ValueAssignments<AnnotatedT>,

    #[resolve(annotations)]
    #[depict(skip)]
    pub(crate) annotations: StructAnnotations,
}

impl<AnnotatedT> Subentity<OperationDefinition<AnnotatedT>> for OperationAssignment<AnnotatedT>
where
    AnnotatedT: 'static + Annotated + Clone + Default,
{
    fn complete(
        &mut self,
        _name: Option<&Name>,
        operation_definition: Option<&OperationDefinition<AnnotatedT>>,
        operation_definition_namespace: Option<&Namespace>,
        context: &mut CompletionContext,
    ) -> Result<(), ToscaError<WithAnnotations>> {
        complete_subentity_map_field!(
            input,
            inputs,
            self,
            operation_definition,
            operation_definition_namespace,
            false,
            context
        );
        complete_subentity_map_field!(
            output,
            outputs,
            self,
            operation_definition,
            operation_definition_namespace,
            false,
            context
        );
        complete_subentity_field!(implementation, self, operation_definition, operation_definition_namespace, context);
        Ok(())
    }
}

impl<AnnotatedT> ToNamespace<OperationAssignment<AnnotatedT>> for OperationDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn to_namespace(&self, namespace: Option<&Namespace>) -> OperationAssignment<AnnotatedT> {
        OperationAssignment {
            implementation: self.implementation.to_namespace(namespace),
            inputs: self.inputs.to_namespace(namespace),
            outputs: self.outputs.to_namespace(namespace),
            annotations: self.annotations.clone_fields(&["implementation", "inputs", "outputs"]),
            ..Default::default()
        }
    }
}

//
// OperationAssignments
//

/// Map of [OperationAssignment].
pub type OperationAssignments<AnnotatedT> = BTreeMap<Name, OperationAssignment<AnnotatedT>>;
