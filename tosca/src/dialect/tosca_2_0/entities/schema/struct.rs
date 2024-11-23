use super::{
    super::{
        super::{super::super::grammar::*, data::*},
        data_type::*,
        schema::*,
    },
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
