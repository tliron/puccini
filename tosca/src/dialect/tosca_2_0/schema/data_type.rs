use super::{
    super::{super::super::grammar::*, data::*, dialect::*, entities::*},
    details::*,
};

use {
    compris::{annotate::*, normal::MalformedError},
    kutil::std::error::*,
};

impl<AnnotatedT> DataType<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    /// Schema validation.
    pub fn schema_validation<SchemaDetailsT>(
        &self,
        definition: &SchemaDetailsT,
        definition_namespace: Option<&Namespace>,
        context: &mut CompletionContext,
    ) -> Result<Option<Expression<AnnotatedT>>, ToscaError<WithAnnotations>>
    where
        AnnotatedT: 'static,
        SchemaDetailsT: SchemaDetails<AnnotatedT>,
    {
        Ok(self.schema(definition, definition_namespace, context)?.and_then(|schema| schema.into_validation()))
    }

    /// Schema.
    pub fn schema<SchemaDetailsT>(
        &self,
        details: &SchemaDetailsT,
        details_namespace: Option<&Namespace>,
        context: &mut CompletionContext,
    ) -> Result<Option<Schema<AnnotatedT>>, ToscaError<WithAnnotations>>
    where
        AnnotatedT: 'static,
        SchemaDetailsT: SchemaDetails<AnnotatedT>,
    {
        let mut schema = Default::default();
        self.initialize_schema(&mut schema, details, details_namespace, context)?;
        Ok(if !schema.is_empty() { Some(schema) } else { None })
    }

    /// Initialize a schema.
    pub fn initialize_schema<SchemaDetailsT>(
        &self,
        schema: &mut Schema<AnnotatedT>,
        details: &SchemaDetailsT,
        details_namespace: Option<&Namespace>,
        context: &mut CompletionContext,
    ) -> Result<Option<SchemaReference>, ToscaError<WithAnnotations>>
    where
        AnnotatedT: 'static,
        SchemaDetailsT: SchemaDetails<AnnotatedT>,
    {
        let schema_key = self.to_schema_key_with_details(Some(details.to_schema_key()));

        // TODO: we won't get everything if it's a fallback
        if let Some(reference) = schema.data_type_references.get(&schema_key) {
            return Ok(Some(*reference));
        }

        // Placeholder
        let placeholder = Default::default();
        schema.data_type_references.insert(schema_key.clone(), placeholder);

        let reference = match self.data_kind {
            Some(DataKind::Scalar) => {
                Some(self.initialize_scalar_schema(schema, placeholder, details, details_namespace))
            }

            Some(DataKind::List) => {
                self.initialize_list_schema(schema, placeholder, details, details_namespace, context)?
            }

            Some(DataKind::Map) => {
                self.initialize_map_schema(schema, placeholder, details, details_namespace, context)?
            }

            Some(DataKind::Struct) => {
                Some(self.initialize_struct_schema(schema, placeholder, details, details_namespace, context)?)
            }

            Some(data_kind) => {
                Some(self.initialize_primitive_schema(schema, placeholder, details, details_namespace, data_kind))
            }

            None => {
                // The placeholder was not used
                schema.data_type_references.remove(&schema_key);

                // A data type without a data kind?
                context.errors.give(
                    MalformedError::new(DATA_TYPE_NAME.into(), "undetermined data kind".into())
                        .with_annotations_from(self),
                )?;
                return Ok(None);
            }
        };

        if let Some(reference) = reference
            && reference != placeholder
        {
            // The reference was used instead of the placeholder
            schema.data_type_references.insert(schema_key.clone(), reference);
            schema.update_reference(placeholder, reference);
        }

        Ok(reference)
    }
}
