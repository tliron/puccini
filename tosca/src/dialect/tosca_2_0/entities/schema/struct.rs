use super::{
    super::{
        super::{super::super::grammar::*, data::*, dialect::*},
        data_type::*,
        property_definition::*,
    },
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
        // TODO: completed_entity (we need and should have error recipient)
        let data_type = catalog.entity::<DataType<AnnotatedT>, _>(DATA_TYPE, &self.type_name, source_id)?;
        let reference = data_type.initialize_schema(&self.type_name, schema, self, source_id, catalog)?;
        Ok(StructSchemaField::new(reference, self.required.unwrap_or(true)))
    }
}
