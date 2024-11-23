use super::{
    super::{
        super::{super::super::grammar::*, data::*, dialect::*},
        attribute_definition::*,
        data_type::*,
    },
    assignment::*,
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
        attribute_definition: Option<(&AttributeDefinition<AnnotatedT>, &Scope)>,
        catalog: &mut Catalog,
        source_id: &SourceID,
        errors: ToscaErrorRecipientRef,
    ) -> Result<(), ToscaError<WithAnnotations>> {
        let Some((attribute_definition, scope)) = attribute_definition else {
            return Ok(());
        };

        let errors = &mut errors.to_error_recipient();

        if self.expression.is_none() && attribute_definition.default.is_some() {
            self.expression = attribute_definition.default.clone();
        }

        if_none_else!(
            data_type,
            self,
            attribute_definition,
            Some(attribute_definition.type_name.clone().in_scope(scope.clone()))
        );

        if let Some(type_name) = &self.data_type
            && let Some(data_type) = catalog
                .completed_entity::<DataType<AnnotatedT>, _, _>(DATA_TYPE, type_name, source_id, errors)?
                .cloned()
        {
            validate_type(&data_type, &attribute_definition.type_name, catalog, errors)?;

            if let Some(validation) = unwrap_or_give!(
                data_type.schema_validation(type_name, attribute_definition, source_id, catalog),
                errors,
                None
            ) {
                self.validation.join_apply(validation);
            }
        }

        if_none_clone!(description, self, attribute_definition);

        Ok(())
    }
}

impl<AnnotatedT> ConvertIntoScope<ValueAssignment<AnnotatedT>> for AttributeDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn convert_into_scope(&self, scope: &Scope) -> ValueAssignment<AnnotatedT> {
        ValueAssignment {
            expression: if self.default.is_some() { self.default.clone() } else { None },
            validation: None,
            data_type: Some(self.type_name.clone().in_scope(scope.clone())),
            metadata: self.metadata.clone(),
            description: self.description.clone(),
            annotations: self.annotations.clone(),
        }
    }
}
