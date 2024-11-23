use super::{
    super::{
        super::{super::super::grammar::*, data::*, dialect::*},
        data_type::*,
        parameter_definition::*,
    },
    assignment::*,
};

use {
    compris::annotate::*,
    kutil::std::{error::*, immutable::*},
};

impl<AnnotatedT> Subentity<ParameterDefinition<AnnotatedT>> for ValueAssignment<AnnotatedT>
where
    AnnotatedT: 'static + Annotated + Clone + Default,
{
    fn complete(
        &mut self,
        name: Option<ByteString>,
        parameter_definition: Option<(&ParameterDefinition<AnnotatedT>, &Scope)>,
        catalog: &mut Catalog,
        source_id: &SourceID,
        errors: ToscaErrorRecipientRef,
    ) -> Result<(), ToscaError<WithAnnotations>> {
        let Some((parameter_definition, scope)) = parameter_definition else {
            return Ok(());
        };

        let errors = &mut errors.to_error_recipient();

        if self.expression.is_none() {
            if parameter_definition.value.is_some() {
                self.expression = parameter_definition.value.clone();
            } else if parameter_definition.default.is_some() {
                self.expression = parameter_definition.default.clone();
            } else if parameter_definition.required.unwrap_or(true) {
                errors.give(MissingRequiredError::new("parameter".into(), name.map(|name| name.into())))?;
            }
        }

        if_none_else!(
            data_type,
            self,
            parameter_definition,
            parameter_definition.type_name.as_ref().map(|type_name| type_name.clone().in_scope(scope.clone()))
        );

        if let Some(type_name) = &self.data_type
            && let Some(data_type) = catalog
                .completed_entity::<DataType<AnnotatedT>, _, _>(DATA_TYPE, type_name, source_id, errors)?
                .cloned()
        {
            if let Some(parent_data_type) = &parameter_definition.type_name {
                validate_type(&data_type, parent_data_type, catalog, errors)?;
            }

            if let Some(validation) = unwrap_or_give!(
                data_type.schema_validation(type_name, parameter_definition, source_id, catalog),
                errors,
                None
            ) {
                self.validation.join_apply(validation);
            }
        }

        if_none_clone!(description, self, parameter_definition);

        Ok(())
    }
}

impl<AnnotatedT> ConvertIntoScope<ValueAssignment<AnnotatedT>> for ParameterDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn convert_into_scope(&self, scope: &Scope) -> ValueAssignment<AnnotatedT> {
        ValueAssignment {
            expression: if self.value.is_some() {
                self.value.clone()
            } else if self.default.is_some() {
                self.default.clone()
            } else {
                None
            },
            validation: None,
            data_type: self.type_name.as_ref().map(|type_name| type_name.clone().in_scope(scope.clone())),
            metadata: self.metadata.clone(),
            description: self.description.clone(),
            annotations: self.annotations.clone(),
        }
    }
}
