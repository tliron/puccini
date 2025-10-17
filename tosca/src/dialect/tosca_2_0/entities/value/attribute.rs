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
        scope: Option<&Scope>,
        attribute_definition: Option<&AttributeDefinition<AnnotatedT>>,
        catalog: &mut Catalog,
        source_id: &SourceID,
        errors: ToscaErrorRecipientRef,
    ) -> Result<(), ToscaError<WithAnnotations>> {
        let errors = &mut errors.to_error_recipient();

        complete_name_field_self_option!(type_name, scope, self, attribute_definition, catalog, errors);

        let Some(attribute_definition) = attribute_definition else {
            return Ok(());
        };

        if self.expression.is_none() && attribute_definition.default.is_some() {
            self.expression = attribute_definition.default.clone();
        }

        complete_field_none_to!(
            type_name,
            self,
            attribute_definition,
            Some(attribute_definition.type_name.into_scoped(scope))
        );

        if let Some(type_name) = &self.type_name
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

        complete_field_none!(description, self, attribute_definition);

        Ok(())
    }
}

impl<AnnotatedT> IntoScoped<ValueAssignment<AnnotatedT>> for AttributeDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn into_scoped(&self, scope: Option<&Scope>) -> ValueAssignment<AnnotatedT> {
        ValueAssignment {
            expression: if self.default.is_some() { self.default.clone() } else { None },
            validation: None,
            type_name: Some(self.type_name.into_scoped(scope)),
            metadata: self.metadata.clone(),
            description: self.description.clone(),
            annotations: self.annotations.clone(),
        }
    }
}
