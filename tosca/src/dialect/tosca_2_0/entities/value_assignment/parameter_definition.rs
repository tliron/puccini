use super::{
    super::{
        super::{super::super::grammar::*, data::*, dialect::*, schema::*},
        data_type::*,
        parameter_definition::*,
    },
    value_assignment::*,
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
        parameter_definition: Option<&ParameterDefinition<AnnotatedT>>,
        paremeter_definition_namespace: Option<&Namespace>,
        context: &mut CompletionContext,
    ) -> Result<(), ToscaError<WithAnnotations>> {
        complete_optional_parent_name_field!(
            type_name,
            paremeter_definition_namespace,
            self,
            parameter_definition,
            context
        );

        let Some(parameter_definition) = parameter_definition else {
            return Ok(());
        };

        if self.expression.is_none() {
            if parameter_definition.value.is_some() {
                self.expression = parameter_definition.value.clone();
            } else if parameter_definition.default.is_some() {
                self.expression = parameter_definition.default.clone();
            } else if parameter_definition.required.unwrap_or(true) {
                context.errors.give(
                    MissingRequiredError::new("parameter".into(), name.map(|name| name.into()))
                        .with_annotations_from(self),
                )?;
            }
        }

        complete_none_field_to!(type_name, self, parameter_definition, || parameter_definition
            .type_name
            .as_ref()
            .map(|type_name| type_name.to_namespace(paremeter_definition_namespace)));

        if let Some(type_name) = &self.type_name
            && let Some(data_type) = context
                .catalog
                .completed_entity::<DataType<AnnotatedT>, _, _>(
                    DATA_TYPE,
                    type_name,
                    context.source_id,
                    &mut context.errors.with_fallback_annotations_from_field(self, "type_name"),
                )?
                .cloned()
        {
            if let Some(parent_data_type) = &parameter_definition.type_name {
                validate_type(&data_type, parent_data_type, context)?;
            }

            if let Some(validation) = unwrap_or_give!(
                data_type.schema_validation(
                    &self.to_schema_key(parameter_definition.type_name.clone()),
                    parameter_definition,
                    context.source_id,
                    context.catalog
                ),
                context.errors,
                None
            ) {
                self.validation.join_apply(validation);
            }
        }

        complete_none_field!(description, self, parameter_definition);

        Ok(())
    }
}

impl<AnnotatedT> ToNamespace<ValueAssignment<AnnotatedT>> for ParameterDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn to_namespace(&self, namespace: Option<&Namespace>) -> ValueAssignment<AnnotatedT> {
        ValueAssignment {
            expression: if self.value.is_some() {
                self.value.clone()
            } else if self.default.is_some() {
                self.default.clone()
            } else {
                None
            },
            validation: None,
            type_name: self.type_name.to_namespace(namespace),
            metadata: self.metadata.clone(),
            description: self.description.clone(),
            annotations: self.annotations.clone(),
        }
    }
}
