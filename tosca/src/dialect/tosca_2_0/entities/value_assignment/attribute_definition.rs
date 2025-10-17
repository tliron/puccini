use super::{
    super::{
        super::{super::super::grammar::*, data::*, dialect::*, schema::*},
        attribute_definition::*,
        data_type::*,
    },
    value_assignment::*,
};

use {
    compris::annotate::*,
    kutil::std::{error::*, immutable::*},
};

impl<AnnotatedT> Subentity<AttributeDefinition<AnnotatedT>> for ValueAssignment<AnnotatedT>
where
    AnnotatedT: 'static + Annotated + Clone + Default,
{
    fn complete(
        &mut self,
        _name: Option<ByteString>,
        attribute_definition: Option<&AttributeDefinition<AnnotatedT>>,
        attribute_definition_namespace: Option<&Namespace>,
        context: &mut CompletionContext,
    ) -> Result<(), ToscaError<WithAnnotations>> {
        complete_optional_name_field!(type_name, self, attribute_definition, attribute_definition_namespace, context);

        let Some(attribute_definition) = attribute_definition else {
            return Ok(());
        };

        if self.expression.is_none() && attribute_definition.default.is_some() {
            self.expression = attribute_definition.default.clone();
        }

        complete_none_field_to!(type_name, self, attribute_definition, || Some(
            attribute_definition.type_name.to_namespace(attribute_definition_namespace)
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
            validate_type(&data_type, &attribute_definition.type_name, context)?;

            if let Some(validation) = unwrap_or_give!(
                data_type.schema_validation(
                    &self.to_schema_key(Some(attribute_definition.type_name.clone())),
                    attribute_definition,
                    context.source_id,
                    context.catalog
                ),
                context.errors,
                None
            ) {
                self.validation.join_apply(validation);
            }
        }

        complete_none_field!(description, self, attribute_definition);

        Ok(())
    }
}

impl<AnnotatedT> ToNamespace<ValueAssignment<AnnotatedT>> for AttributeDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn to_namespace(&self, namespace: Option<&Namespace>) -> ValueAssignment<AnnotatedT> {
        ValueAssignment {
            expression: if self.default.is_some() { self.default.clone() } else { None },
            validation: None,
            type_name: Some(self.type_name.to_namespace(namespace)),
            metadata: self.metadata.clone(),
            description: self.description.clone(),
            annotations: self.annotations.clone(),
        }
    }
}
