use super::{
    super::super::super::grammar::*, interface_definition::*, notification_assignment::*, operation_assignment::*,
    value::*,
};

use {
    compris::{annotate::*, resolve::*},
    kutil::{
        cli::depict::*,
        std::{error::*, immutable::*},
    },
    std::collections::*,
};

//
// InterfaceAssignment
//

/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
///
/// An interface assignment is used to specify assignments for the inputs, operations and
/// notifications defined in the interface. Interface assignments may be used within a node or
/// relationship template definition (including when interface assignments are referenced as part
/// of a requirement assignment in a node template).
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
        scope: Option<&Scope>,
        interface_definition: Option<&InterfaceDefinition<AnnotatedT>>,
        catalog: &mut Catalog,
        source_id: &SourceID,
        errors: ToscaErrorRecipientRef,
    ) -> Result<(), ToscaError<WithAnnotations>> {
        let errors = &mut errors.to_error_recipient();

        complete_subentity_map_field!(
            input,
            inputs,
            scope,
            self,
            interface_definition,
            true,
            catalog,
            source_id,
            errors
        );
        complete_subentity_map_field!(
            operation,
            operations,
            scope,
            self,
            interface_definition,
            true,
            catalog,
            source_id,
            errors
        );
        complete_subentity_map_field!(
            notification,
            notifications,
            scope,
            self,
            interface_definition,
            true,
            catalog,
            source_id,
            errors
        );

        Ok(())
    }
}

impl<AnnotatedT> IntoScoped<InterfaceAssignment<AnnotatedT>> for InterfaceDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn into_scoped(&self, scope: Option<&Scope>) -> InterfaceAssignment<AnnotatedT> {
        InterfaceAssignment {
            inputs: self.inputs.into_scoped(scope),
            operations: self.operations.into_scoped(scope),
            notifications: self.notifications.into_scoped(scope),
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
