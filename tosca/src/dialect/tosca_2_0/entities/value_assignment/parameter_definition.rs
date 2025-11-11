use super::{
    super::{
        super::{super::super::grammar::*, data::*, dialect::*},
        data_type::*,
        parameter_definition::*,
    },
    value_assignment::*,
};

use {compris::annotate::*, kutil::std::error::*};

impl<AnnotatedT> Subentity<ParameterDefinition<AnnotatedT>> for ValueAssignment<AnnotatedT>
where
    AnnotatedT: 'static + Annotated + Clone + Default,
{
    fn complete(
        &mut self,
        name: Option<&Name>,
        parameter_definition: Option<&ParameterDefinition<AnnotatedT>>,
        parameter_definition_namespace: Option<&Namespace>,
        context: &mut CompletionContext,
    ) -> Result<(), ToscaError<WithAnnotations>> {
        complete_optional_parent_type_name_field!(
            type_name,
            self,
            parameter_definition,
            parameter_definition_namespace,
            false,
            context
        );

        let Some(parameter_definition) = parameter_definition else {
            return Ok(());
        };

        if self.expression.is_none() {
            if parameter_definition.value.is_some() {
                self.expression = parameter_definition.value.to_namespace(parameter_definition_namespace);
            } else if parameter_definition.default.is_some() {
                self.expression = parameter_definition.default.to_namespace(parameter_definition_namespace);
            } else if parameter_definition.required.unwrap_or(true) {
                context.errors.give(
                    MissingRequiredError::new("parameter".into(), name.map(|name| name.to_string()))
                        .with_annotations_from(self),
                )?;
            }
        }

        let (data_type, _data_type_namespace) =
            completed_entity_from_optional_full_name_field!(DATA_TYPE, DataType, self, type_name, context);

        if let Some(data_type) = data_type {
            if let Some(parent_data_type) = &parameter_definition.type_name {
                validate_type(&data_type, parent_data_type, context)?;
            }

            if let Some(validation) = unwrap_or_give!(
                data_type.schema_validation(parameter_definition, parameter_definition_namespace, context),
                context.errors,
                None
            ) {
                self.validation.join_apply(validation);
            }
        }

        complete_optional_field!(description, self, parameter_definition);

        Ok(())
    }
}

impl<AnnotatedT> ToNamespace<ValueAssignment<AnnotatedT>> for ParameterDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn to_namespace(&self, namespace: Option<&Namespace>) -> ValueAssignment<AnnotatedT> {
        ValueAssignment {
            // expression: if self.value.is_some() {
            //     self.value.clone()
            // } else if self.default.is_some() {
            //     self.default.clone()
            // } else {
            //     None
            // },
            expression: None,
            validation: None,
            type_name: self.type_name.to_namespace(namespace),
            metadata: self.metadata.clone(),
            description: self.description.clone(),
            annotations: self.annotations.clone(),
        }
    }
}
