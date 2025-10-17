use super::{
    super::{super::super::grammar::*, data::*, dialect::*, entities::*},
    details::*,
    macros::*,
};

use compris::annotate::*;

impl<AnnotatedT> DataType<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    /// Initialize struct schema.
    pub fn initialize_struct_schema<SchemaDetailsT>(
        &self,
        schema: &mut Schema<AnnotatedT>,
        reference: SchemaReference,
        definition: &SchemaDetailsT,
        source_id: &SourceID,
        catalog: &Catalog,
    ) -> Result<SchemaReference, ToscaError<WithAnnotations>>
    where
        AnnotatedT: 'static,
        SchemaDetailsT: SchemaDetails<AnnotatedT>,
    {
        let mut struct_schema = StructSchema::default();

        complete_schema_default_and_validation!(struct_schema, self, definition);

        if let Some(properties) = &self.properties {
            for (name, property) in properties {
                let field = property.initialize_struct_field_schema(schema, source_id, catalog)?;
                struct_schema.fields.insert(name.clone(), field);
            }
        }

        Ok(schema.add_unique(reference, struct_schema.into()))
    }
}

impl<AnnotatedT> PropertyDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    /// Initialize struct field schema.
    pub fn initialize_struct_field_schema(
        &self,
        schema: &mut Schema<AnnotatedT>,
        source_id: &SourceID,
        catalog: &Catalog,
    ) -> Result<StructSchemaField, ToscaError<WithAnnotations>>
    where
        AnnotatedT: 'static,
    {
        // TODO: we should have error receiver?
        let data_type = catalog
            .entity::<DataType<AnnotatedT>, _>(DATA_TYPE, &self.type_name, source_id)
            .map_err(|error| error.with_annotations_from_field(self, "type_name"))?;

        let reference = data_type.initialize_schema(
            &self.to_schema_key(Some(self.type_name.clone())),
            schema,
            self,
            source_id,
            catalog,
        )?;

        Ok(StructSchemaField::new(reference, self.required.unwrap_or(true)))
    }
}
