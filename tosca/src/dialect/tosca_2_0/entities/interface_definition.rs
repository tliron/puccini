use super::{
    super::{super::super::grammar::*, dialect::*},
    interface_type::*,
    notification_definition::*,
    operation_definition::*,
    parameter_definition::*,
};

use {
    compris::{annotate::*, resolve::*},
    depiction::*,
    kutil::std::immutable::*,
    std::collections::*,
};

//
// InterfaceDefinition
//

/// An interface definition defines an interface (containing operations and notifications
/// definitions) that can be associated with (i.e. defined within) a node or relationship type
/// definition. An interface definition may be refined in subsequent node or relationship type
/// derivations.
///
/// (Documentation copied from
/// [TOSCA specification 2.0](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html))
#[derive(Clone, Debug, Default, Depict, Resolve)]
#[depict(tag = tag::source_and_span)]
#[resolve(annotated_parameter=AnnotatedT)]
pub struct InterfaceDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    /// The mandatory name of the interface type on which this interface definition is based.
    ///
    /// Puccini note: *Not* mandatory, as it can be inherited from parent.
    #[resolve(key = "type")]
    #[depict(as(depict))]
    pub type_name: FullName,

    /// The optional description for this interface definition.
    #[resolve]
    #[depict(option, style(string))]
    pub description: Option<ByteString>,

    /// Defines additional information.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub metadata: Metadata<AnnotatedT>,

    /// The optional map of input parameter refinements and new input parameter definitions
    /// available to all operations defined for this interface (the input parameters to be
    /// refined have been defined in the interface type definition).
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub inputs: ParameterDefinitions<AnnotatedT>,

    /// The optional map of operations refinements for this interface. The referred operations
    /// must have been defined in the interface type definition.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub operations: OperationDefinitions<AnnotatedT>,

    /// The optional map of notifications refinements for this interface. The referred operations
    /// must have been defined in the interface type definition.
    #[resolve]
    #[depict(iter(kv), as(depict), key_style(string))]
    pub notifications: NotificationDefinitions<AnnotatedT>,

    #[resolve(annotations)]
    #[depict(skip)]
    pub(crate) annotations: StructAnnotations,
}

impl<AnnotatedT> Subentity<Self> for InterfaceDefinition<AnnotatedT>
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
        complete_type_name_field!(self, parent, parent_namespace, true, context);
        complete_subentity_map_field!(input, inputs, self, parent, parent_namespace, true, context);
        complete_subentity_map_field!(operation, operations, self, parent, parent_namespace, true, context);
        complete_subentity_map_field!(notification, notifications, self, parent, parent_namespace, true, context);

        let (interface_type, interface_type_namespace) =
            completed_entity_from_full_name_field!(INTERFACE_TYPE, InterfaceType, self, type_name, context);

        complete_subentity_map_field!(input, inputs, self, interface_type, interface_type_namespace, true, context);
        complete_subentity_map_field!(
            operation,
            operations,
            self,
            interface_type,
            interface_type_namespace,
            true,
            context
        );
        complete_subentity_map_field!(
            notification,
            notifications,
            self,
            interface_type,
            interface_type_namespace,
            true,
            context
        );

        Ok(())
    }
}

impl<AnnotatedT> ToNamespace<Self> for InterfaceDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn to_namespace(&self, namespace: Option<&Namespace>) -> Self {
        Self {
            type_name: self.type_name.to_namespace(namespace),
            description: self.description.clone(),
            metadata: self.metadata.clone(),
            inputs: self.inputs.to_namespace(namespace),
            operations: self.operations.to_namespace(namespace),
            notifications: self.notifications.to_namespace(namespace),
            annotations: self.annotations.clone(),
        }
    }
}

//
// InterfaceDefinitions
//

/// Map of [InterfaceDefinition].
pub type InterfaceDefinitions<AnnotatedT> = BTreeMap<Name, InterfaceDefinition<AnnotatedT>>;
