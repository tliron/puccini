use super::super::{
    super::{super::super::grammar::*, dialect::*},
    value::*,
};

use {
    compris::annotate::*,
    kutil::std::{error::*, immutable::*},
    std::collections::*,
};

impl<AnnotatedT> ValueAssignment<AnnotatedT> {
    /// Compile to Floria.
    pub fn compile<ErrorRecipientT>(
        &self,
        tosca_entity: &'static str,
        read_only: bool,
        directory: &floria::Directory,
        store: floria::StoreRef,
        errors: &mut ErrorRecipientT,
    ) -> Result<floria::Property, ToscaError<AnnotatedT>>
    where
        AnnotatedT: Annotated + Clone + Default,
        ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>,
    {
        let (preparer, updater, value) = self.floria_property_fields();
        let mut floria_property = floria::Property::new(read_only, preparer, updater, value);

        floria_property.metadata.set_tosca_entity_static(DIALECT_ID, tosca_entity);
        floria_property.metadata.set_tosca_description(self.description.as_ref());
        floria_property.metadata.set_tosca_custom_metadata(&self.metadata);

        if let Some(data_type) = &self.data_type {
            floria_property.class_ids.add_tosca_type(data_type, directory, store.clone(), errors)?;
        }

        Ok(floria_property)
    }

    /// Floria property preparer, updater, and value.
    pub fn floria_property_fields(
        &self,
    ) -> (Option<floria::Expression>, Option<floria::Expression>, Option<floria::Expression>)
    where
        AnnotatedT: Annotated + Clone + Default,
    {
        let preparer = self.validation.clone().map(|validation| validation.into());

        let (updater, value) = self
            .expression
            .clone()
            .map(|expression| {
                let expression: floria::Expression = expression.into();
                if expression.is_literal() { (None, Some(expression)) } else { (Some(expression), None) }
            })
            .unwrap_or_default();

        (preparer, updater, value)
    }
}

/// Compile value assignments.
pub fn compile_value_assignments<AnnotatedT, ErrorRecipientT>(
    property_templates: &mut BTreeMap<ByteString, floria::Property>,
    value_assignments: &ValueAssignments<AnnotatedT>,
    tosca_entity: &'static str,
    read_only: bool,
    directory: &floria::Directory,
    store: floria::StoreRef,
    errors: &mut ErrorRecipientT,
) -> Result<(), ToscaError<AnnotatedT>>
where
    AnnotatedT: Annotated + Clone + Default,
    ErrorRecipientT: ErrorRecipient<ToscaError<AnnotatedT>>,
{
    for (name, value_assignment) in value_assignments {
        property_templates
            .insert(name.clone(), value_assignment.compile(tosca_entity, read_only, directory, store.clone(), errors)?);
    }
    Ok(())
}
