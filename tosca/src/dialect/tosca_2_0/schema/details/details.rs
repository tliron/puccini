use super::super::{
    super::{super::super::grammar::*, data::*, entities::*},
    macros::*,
};

use compris::annotate::*;

//
// SchemaDetails
//

/// Schema details.
pub trait SchemaDetails<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    /// Default expression.
    ///
    /// (Not named "default" to avoid ambiguity with [Default::default].)
    fn default_expression(&self) -> Option<&Expression<AnnotatedT>>;

    /// Validation.
    fn validation(&self) -> Option<&Expression<AnnotatedT>>;

    /// Key schema.
    fn key_schema(&self) -> Option<&SchemaDefinition<AnnotatedT>>;

    /// Entry schema.
    fn entry_schema(&self) -> Option<&SchemaDefinition<AnnotatedT>>;

    /// To schema key.
    fn to_schema_key(&self, data_type: Option<FullName>) -> SchemaKey {
        SchemaKey::new(
            data_type,
            self.validation().map(|validation| validation.remove_annotations()),
            self.key_schema().map(|schema_definition| {
                schema_definition.to_schema_key(Some(schema_definition.type_name.clone())).into()
            }),
            self.entry_schema().map(|schema_definition| {
                schema_definition.to_schema_key(Some(schema_definition.type_name.clone())).into()
            }),
        )
    }

    /// Initialize primitive schema.
    fn initialize_primitive_schema<SchemaDetailsT>(
        &self,
        schema: &mut Schema<AnnotatedT>,
        reference: SchemaReference,
        definition: &SchemaDetailsT,
        data_kind: DataKind,
    ) -> Result<SchemaReference, ToscaError<WithAnnotations>>
    where
        SchemaDetailsT: SchemaDetails<AnnotatedT>,
    {
        let mut primitive_schema: PrimitiveSchema<_> = data_kind.into();
        complete_schema_default_and_validation!(primitive_schema, self, definition);
        Ok(schema.add_unique(reference, primitive_schema.into()))
    }

    /// Initialize list schema.
    fn initialize_list_schema<SchemaDetailsT>(
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
        let mut list_schema = ListSchema::default();

        complete_complex_schema_default_and_validation!(list_schema, self, definition);
        complete_entry_schema!(list_schema, self, definition, schema, source_id, catalog);

        Ok(schema.add_unique(reference, list_schema.into()))
    }

    /// Initialize map schema.
    fn initialize_map_schema<SchemaDetailsT>(
        &self,
        schema: &mut Schema<AnnotatedT>,
        reference: SchemaReference,
        schema_details: &SchemaDetailsT,
        source_id: &SourceID,
        catalog: &Catalog,
    ) -> Result<SchemaReference, ToscaError<WithAnnotations>>
    where
        AnnotatedT: 'static,
        SchemaDetailsT: SchemaDetails<AnnotatedT>,
    {
        let mut map_schema = MapSchema::default();

        complete_complex_schema_default_and_validation!(map_schema, self, schema_details);
        complete_key_schema!(map_schema, self, schema_details, schema, source_id, catalog);
        complete_entry_schema!(map_schema, self, schema_details, schema, source_id, catalog);

        Ok(schema.add_unique(reference, map_schema.into()))
    }
}
