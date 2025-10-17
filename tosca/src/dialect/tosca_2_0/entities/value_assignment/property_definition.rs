use super::{
    super::{
        super::{super::super::grammar::*, data::*, dialect::*, schema::*},
        data_type::*,
        property_definition::*,
    },
    value_assignment::*,
};

use {
    compris::annotate::*,
    kutil::std::{error::*, immutable::*},
};

impl<AnnotatedT> Subentity<PropertyDefinition<AnnotatedT>> for ValueAssignment<AnnotatedT>
where
    AnnotatedT: 'static + Annotated + Clone + Default,
{
    fn complete(
        &mut self,
        name: Option<ByteString>,
        property_definition: Option<&PropertyDefinition<AnnotatedT>>,
        property_definition_namespace: Option<&Namespace>,
        context: &mut CompletionContext,
    ) -> Result<(), ToscaError<WithAnnotations>> {
        complete_optional_name_field!(type_name, self, property_definition, property_definition_namespace, context);

        let Some(property_definition) = property_definition else {
            return Ok(());
        };

        if let Some(expression) = &self.expression {
            if property_definition.value.is_some() {
                context.errors.give(OverrideProhibitedError::new("value".into()).with_annotations_from(expression))?;
            }
        } else if property_definition.value.is_some() {
            self.expression = property_definition.value.clone();
        } else if property_definition.default.is_some() {
            self.expression = property_definition.default.clone();
        } else if property_definition.required.unwrap_or(true) {
            context.errors.give(
                MissingRequiredError::new("property".into(), name.map(|name| name.into())).with_annotations_from(self),
            )?;
        }

        complete_none_field_to!(type_name, self, property_definition, || Some(
            property_definition.type_name.to_namespace(property_definition_namespace)
        ));

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
            validate_type(&data_type, &property_definition.type_name, context)?;

            if let Some(validation) = unwrap_or_give!(
                data_type.schema_validation(
                    &self.to_schema_key(Some(property_definition.type_name.clone())),
                    property_definition,
                    context.source_id,
                    context.catalog
                ),
                context.errors,
                None
            ) {
                self.validation.join_apply(validation);
            }
        }

        complete_none_field!(description, self, property_definition);

        Ok(())
    }
}

impl<AnnotatedT> ToNamespace<ValueAssignment<AnnotatedT>> for PropertyDefinition<AnnotatedT>
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
            type_name: Some(self.type_name.to_namespace(namespace)),
            metadata: self.metadata.clone(),
            description: self.description.clone(),
            annotations: self.annotations.clone(),
        }
    }
}
