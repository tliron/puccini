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
        scope: Option<&Scope>,
        property_definition: Option<&PropertyDefinition<AnnotatedT>>,
        catalog: &mut Catalog,
        source_id: &SourceID,
        errors: ToscaErrorRecipientRef,
    ) -> Result<(), ToscaError<WithAnnotations>> {
        let errors = &mut errors.to_error_recipient();

        complete_name_field_self_option!(type_name, scope, self, property_definition, catalog, errors);

        let Some(property_definition) = property_definition else {
            return Ok(());
        };

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

        complete_field_none_to!(
            type_name,
            self,
            property_definition,
            Some(property_definition.type_name.into_scoped(scope))
        );

        if let Some(type_name) = &self.type_name
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

        complete_field_none!(description, self, property_definition);

        Ok(())
    }
}

impl<AnnotatedT> IntoScoped<ValueAssignment<AnnotatedT>> for PropertyDefinition<AnnotatedT>
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
            type_name: Some(self.type_name.into_scoped(scope)),
            metadata: self.metadata.clone(),
            description: self.description.clone(),
            annotations: self.annotations.clone(),
        }
    }
}
