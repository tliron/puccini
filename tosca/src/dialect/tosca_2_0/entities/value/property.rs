use super::{
    super::{
        super::{super::super::grammar::*, data::*, dialect::*},
        data_type::*,
        property_definition::*,
    },
    assignment::*,
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
        property_definition: Option<(&PropertyDefinition<AnnotatedT>, &Scope)>,
        catalog: &mut Catalog,
        source_id: &SourceID,
        errors: ToscaErrorRecipientRef,
    ) -> Result<(), ToscaError<WithAnnotations>> {
        let Some((property_definition, scope)) = property_definition else {
            return Ok(());
        };

        let errors = &mut errors.to_error_recipient();

        if self.expression.is_some() {
            if property_definition.value.is_some() {
                errors.give(OverrideProhibitedError::new("value".into()))?;
            }
        } else if property_definition.value.is_some() {
            self.expression = property_definition.value.clone();
        } else if property_definition.default.is_some() {
            self.expression = property_definition.default.clone();
        } else if property_definition.required.unwrap_or(true) {
            errors.give(MissingRequiredError::new("property".into(), name.map(|name| name.into())))?;
        }

        if_none_else!(
            data_type,
            self,
            property_definition,
            Some(property_definition.type_name.clone().in_scope(scope.clone()))
        );

        if let Some(type_name) = &self.data_type
            && let Some(data_type) = catalog
                .completed_entity::<DataType<AnnotatedT>, _, _>(DATA_TYPE, type_name, source_id, errors)?
                .cloned()
        {
            validate_type(&data_type, &property_definition.type_name, catalog, errors)?;

            if let Some(validation) = unwrap_or_give!(
                data_type.schema_validation(type_name, property_definition, source_id, catalog),
                errors,
                None
            ) {
                self.validation.join_apply(validation);
            }
        }

        if_none_clone!(description, self, property_definition);

        Ok(())
    }
}

impl<AnnotatedT> ConvertIntoScope<ValueAssignment<AnnotatedT>> for PropertyDefinition<AnnotatedT>
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
            data_type: Some(self.type_name.clone().in_scope(scope.clone())),
            metadata: self.metadata.clone(),
            description: self.description.clone(),
            annotations: self.annotations.clone(),
        }
    }
}
