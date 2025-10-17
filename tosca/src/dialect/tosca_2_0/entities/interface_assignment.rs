use super::{
    super::super::super::grammar::*, interface_definition::*, notification_assignment::*, operation_assignment::*,
    value_assignment::*,
};

use {
    compris::{annotate::*, resolve::*},
    depiction::*,
    kutil::std::immutable::*,
    std::collections::*,
};

//
// InterfaceAssignment
//

/// An interface assignment is used to specify assignments for the inputs, operations and
/// notifications defined in the interface. Interface assignments may be used within a node or
/// relationship template definition (including when interface assignments are referenced as part
/// of a requirement assignment in a node template).
///
/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
#[derive(Clone, Debug, Default, Depict, Resolve)]
#[depict(tag = tag::source_and_span)]
#[resolve(annotated_parameter=AnnotatedT)]
pub struct InterfaceAssignment<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    /// The optional map of input parameter assignments. Template authors MAY provide parameter
    /// assignments for interface inputs that are not defined in their corresponding interface
    /// type.
    #[resolve]
    #[depict(iter(kv), as(depict), key_as(display), key_style(name))]
    pub inputs: ValueAssignments<AnnotatedT>,

    /// The optional map of operations assignments specified for this interface.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub operations: OperationAssignments<AnnotatedT>,

    /// The optional map of notifications assignments specified for this interface.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub notifications: NotificationAssignments<AnnotatedT>,

    #[resolve(annotations)]
    #[depict(skip)]
    pub(crate) annotations: StructAnnotations,
}

impl<AnnotatedT> Subentity<InterfaceDefinition<AnnotatedT>> for InterfaceAssignment<AnnotatedT>
where
    AnnotatedT: 'static + Annotated + Clone + Default,
{
    fn complete(
        &mut self,
        _name: Option<ByteString>,
        interface_definition: Option<&InterfaceDefinition<AnnotatedT>>,
        interface_definition_namespace: Option<&Namespace>,
        context: &mut CompletionContext,
    ) -> Result<(), ToscaError<WithAnnotations>> {
        complete_subentity_map_field!(
            input,
            inputs,
            self,
            interface_definition,
            interface_definition_namespace,
            true,
            context
        );
        complete_subentity_map_field!(
            operation,
            operations,
            self,
            interface_definition,
            interface_definition_namespace,
            true,
            context
        );
        complete_subentity_map_field!(
            notification,
            notifications,
            self,
            interface_definition,
            interface_definition_namespace,
            true,
            context
        );
        Ok(())
    }
}

impl<AnnotatedT> ToNamespace<InterfaceAssignment<AnnotatedT>> for InterfaceDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn to_namespace(&self, namespace: Option<&Namespace>) -> InterfaceAssignment<AnnotatedT> {
        InterfaceAssignment {
            inputs: self.inputs.to_namespace(namespace),
            operations: self.operations.to_namespace(namespace),
            notifications: self.notifications.to_namespace(namespace),
            annotations: self.annotations.clone_fields(&["inputs", "operations", "notifications"]),
            ..Default::default()
        }
    }
}

//
// InterfaceAssignments
//

/// Map of [InterfaceAssignment].
pub type InterfaceAssignments<AnnotatedT> = BTreeMap<ByteString, InterfaceAssignment<AnnotatedT>>;
