use super::{
    super::{
        super::{super::super::grammar::*, data::*, dialect::*},
        data_type::*,
        property_definition::*,
    },
    value_assignment::*,
};

use {compris::annotate::*, problemo::*};

impl<AnnotatedT> Subentity<PropertyDefinition<AnnotatedT>> for ValueAssignment<AnnotatedT>
where
    AnnotatedT: 'static + Annotated + Clone + Default,
{
    fn complete(
        &mut self,
        name: Option<&Name>,
        property_definition: Option<&PropertyDefinition<AnnotatedT>>,
        property_definition_namespace: Option<&Namespace>,
        context: &mut CompletionContext,
    ) -> Result<(), Problem> {
        complete_optional_type_name_field!(self, property_definition, property_definition_namespace, true, context);

        let Some(property_definition) = property_definition else {
            return Ok(());
        };

        if let Some(expression) = &self.expression {
            if property_definition.value.is_some() {
                context
                    .problems
                    .give(OverrideProhibitedError::as_problem("value").with_annotations_from(expression))?;
            }
        } else if property_definition.value.is_some() {
            self.expression = property_definition.value.to_namespace(property_definition_namespace);
        } else if property_definition.default.is_some() {
            self.expression = property_definition.default.to_namespace(property_definition_namespace);
        } else if property_definition.required.unwrap_or(true) {
            context.problems.give(MissingRequiredError::as_problem("property", name).with_annotations_from(self))?;
        }

        let (data_type, _data_type_namespace) =
            completed_entity_from_optional_full_name_field!(DATA_TYPE, DataType, self, type_name, context);

        if let Some(data_type) = data_type {
            validate_type(&data_type, &property_definition.type_name, context)?;

            if let Some(validation) = give_unwrap!(
                data_type.schema_validation(property_definition, property_definition_namespace, context),
                &mut context.problems,
            ) {
                self.validation.join_apply(validation);
            }
        }

        complete_optional_field!(description, self, property_definition);

        Ok(())
    }
}

impl<AnnotatedT> ToNamespace<ValueAssignment<AnnotatedT>> for PropertyDefinition<AnnotatedT>
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
            type_name: Some(self.type_name.to_namespace(namespace)),
            metadata: self.metadata.clone(),
            description: self.description.clone(),
            annotations: self.annotations.clone(),
        }
    }
}
