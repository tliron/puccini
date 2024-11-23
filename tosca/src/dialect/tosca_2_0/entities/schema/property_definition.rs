use super::super::{
    super::{super::super::grammar::*, data::*, dialect::*},
    data_type::*,
    property_definition::*,
};

use compris::annotate::*;

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
        let data_type = catalog.entity::<DataType<AnnotatedT>, _>(DATA_TYPE, &self.type_name, source_id)?;
        let reference = data_type.initialize_schema(&self.type_name, schema, self, source_id, catalog)?;
        Ok(StructSchemaField::new(reference, self.required.unwrap_or(true)))
    }
}
