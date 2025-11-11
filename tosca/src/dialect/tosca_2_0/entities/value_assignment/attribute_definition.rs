use super::{
    super::{
        super::{super::super::grammar::*, data::*, dialect::*},
        attribute_definition::*,
        data_type::*,
    },
    value_assignment::*,
};

use {compris::annotate::*, kutil::std::error::*};

impl<AnnotatedT> Subentity<AttributeDefinition<AnnotatedT>> for ValueAssignment<AnnotatedT>
where
    AnnotatedT: 'static + Annotated + Clone + Default,
{
    fn complete(
        &mut self,
        _name: Option<&Name>,
        attribute_definition: Option<&AttributeDefinition<AnnotatedT>>,
        attribute_definition_namespace: Option<&Namespace>,
        context: &mut CompletionContext,
    ) -> Result<(), ToscaError<WithAnnotations>> {
        complete_optional_type_name_field!(self, attribute_definition, attribute_definition_namespace, true, context);

        let Some(attribute_definition) = attribute_definition else {
            return Ok(());
        };

        if self.expression.is_none() && attribute_definition.default.is_some() {
            self.expression = attribute_definition.default.to_namespace(attribute_definition_namespace);
        }

        let (data_type, _data_type_namespace) =
            completed_entity_from_optional_full_name_field!(DATA_TYPE, DataType, self, type_name, context);

        if let Some(data_type) = data_type {
            validate_type(&data_type, &attribute_definition.type_name, context)?;

            if let Some(validation) = unwrap_or_give!(
                data_type.schema_validation(attribute_definition, attribute_definition_namespace, context),
                context.errors,
                None
            ) {
                self.validation.join_apply(validation);
            }
        }

        complete_optional_field!(description, self, attribute_definition);

        Ok(())
    }
}

impl<AnnotatedT> ToNamespace<ValueAssignment<AnnotatedT>> for AttributeDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn to_namespace(&self, namespace: Option<&Namespace>) -> ValueAssignment<AnnotatedT> {
        ValueAssignment {
            // expression: if self.default.is_some() { self.default.clone() } else { None },
            expression: None,
            validation: None,
            type_name: Some(self.type_name.to_namespace(namespace)),
            metadata: self.metadata.clone(),
            description: self.description.clone(),
            annotations: self.annotations.clone(),
        }
    }
}
