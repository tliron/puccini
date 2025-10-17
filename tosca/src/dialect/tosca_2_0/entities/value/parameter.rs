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
        scope: Option<&Scope>,
        parameter_definition: Option<&ParameterDefinition<AnnotatedT>>,
        catalog: &mut Catalog,
        source_id: &SourceID,
        errors: ToscaErrorRecipientRef,
    ) -> Result<(), ToscaError<WithAnnotations>> {
        let errors = &mut errors.to_error_recipient();

        complete_name_field_both_option!(type_name, scope, self, parameter_definition, catalog, errors);

        let Some(parameter_definition) = parameter_definition else {
            return Ok(());
        };

        if self.expression.is_none() {
            if parameter_definition.value.is_some() {
                self.expression = parameter_definition.value.clone();
            } else if parameter_definition.default.is_some() {
                self.expression = parameter_definition.default.clone();
            } else if parameter_definition.required.unwrap_or(true) {
                errors.give(MissingRequiredError::new("parameter".into(), name.map(|name| name.into())))?;
            }
        }

        complete_field_none_to!(
            type_name,
            self,
            parameter_definition,
            parameter_definition.type_name.as_ref().map(|type_name| type_name.into_scoped(scope))
        );

        if let Some(type_name) = &self.type_name
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

        complete_field_none!(description, self, parameter_definition);

        Ok(())
    }
}

impl<AnnotatedT> IntoScoped<ValueAssignment<AnnotatedT>> for ParameterDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn into_scoped(&self, scope: Option<&Scope>) -> ValueAssignment<AnnotatedT> {
        ValueAssignment {
            expression: if self.value.is_some() {
                self.value.clone()
            } else if self.default.is_some() {
                self.default.clone()
            } else {
                None
            },
            validation: None,
            type_name: self.type_name.into_scoped(scope),
            metadata: self.metadata.clone(),
            description: self.description.clone(),
            annotations: self.annotations.clone(),
        }
    }
}
