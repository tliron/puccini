use super::super::{
    super::{super::super::grammar::*, data::*},
    data_type::*,
    schema::*,
};

use compris::{annotate::*, normal::MalformedError};

impl<AnnotatedT> DataType<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    /// Schema validation.
    pub fn schema_validation<SchemaDetailsT>(
        &self,
        data_type_name: &FullName,
        definition: &SchemaDetailsT,
        source_id: &SourceID,
        catalog: &Catalog,
    ) -> Result<Option<Expression<AnnotatedT>>, ToscaError<WithAnnotations>>
    where
        AnnotatedT: 'static,
        SchemaDetailsT: SchemaDetails<AnnotatedT>,
    {
        Ok(self.schema(data_type_name, definition, source_id, catalog)?.and_then(|schema| schema.into_validation()))
    }

    /// Schema.
    pub fn schema<SchemaDetailsT>(
        &self,
        data_type_name: &FullName,
        definition: &SchemaDetailsT,
        source_id: &SourceID,
        catalog: &Catalog,
    ) -> Result<Option<Schema<AnnotatedT>>, ToscaError<WithAnnotations>>
    where
        AnnotatedT: 'static,
        SchemaDetailsT: SchemaDetails<AnnotatedT>,
    {
        let mut schema = Default::default();
        self.initialize_schema(data_type_name, &mut schema, definition, source_id, catalog)?;
        Ok(if !schema.is_empty() { Some(schema) } else { None })
    }

    /// Initialize a schema.
    pub fn initialize_schema<SchemaDetailsT>(
        &self,
        data_type_name: &FullName,
        schema: &mut Schema<AnnotatedT>,
        definition: &SchemaDetailsT,
        source_id: &SourceID,
        catalog: &Catalog,
    ) -> Result<SchemaReference, ToscaError<WithAnnotations>>
    where
        AnnotatedT: 'static,
        SchemaDetailsT: SchemaDetails<AnnotatedT>,
    {
        // TODO: we won't get everything
        if let Some(reference) = schema.data_type_references.get(data_type_name) {
            return Ok(*reference);
        }

        // Placeholder
        let placeholder = Default::default();
        schema.data_type_references.insert(data_type_name.clone(), placeholder);

        let reference = match self.data_kind {
            Some(DataKind::Scalar) => self.initialize_scalar_schema(schema, placeholder, definition)?,

            Some(DataKind::List) => self.initialize_list_schema(schema, placeholder, definition, source_id, catalog)?,

            Some(DataKind::Map) => self.initialize_map_schema(schema, placeholder, definition, source_id, catalog)?,

            Some(DataKind::Struct) => {
                self.initialize_struct_schema(schema, placeholder, definition, source_id, catalog)?
            }

            Some(data_kind) => self.initialize_primitive_schema(schema, placeholder, definition, data_kind)?,

            None => {
                // The placeholder was not used
                schema.data_type_references.remove(data_type_name);

                // A data type without a data kind?
                return Err(MalformedError::new("data type".into(), "undetermined data kind".into())
                    .with_annotations_from(self)
                    .into());
            }
        };

        if reference != placeholder {
            // The reference was used instead of the placeholder
            schema.data_type_references.insert(data_type_name.clone(), reference);
            schema.update_reference(placeholder, reference);
        }

        Ok(reference)
    }
}
