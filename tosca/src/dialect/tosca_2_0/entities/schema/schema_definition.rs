use super::super::{
    super::{super::super::grammar::*, data::*, dialect::*},
    data_type::*,
    schema_definition::*,
};

use compris::annotate::*;

impl<AnnotatedT> SchemaDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    /// Initialize a schema.
    pub fn initialize_schema(
        &self,
        schema: &mut Schema<AnnotatedT>,
        source_id: &SourceID,
        catalog: &Catalog,
    ) -> Result<SchemaReference, ToscaError<WithAnnotations>>
    where
        AnnotatedT: 'static,
    {
        let data_type = catalog.entity::<DataType<AnnotatedT>, _>(DATA_TYPE, &self.type_name, source_id)?;
        let reference = data_type.initialize_schema(&self.type_name, schema, self, source_id, catalog)?;
        Ok(reference.into())
    }
}
