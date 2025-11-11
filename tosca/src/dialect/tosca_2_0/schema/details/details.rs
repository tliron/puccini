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
    /// Data type name.
    fn data_type_name(&self) -> Option<&FullName>;

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
    fn to_schema_key(&self) -> SchemaKey {
        self.to_schema_key_with_details(None)
    }

    /// To schema key.
    fn to_schema_key_with_details(&self, details_schema_key: Option<SchemaKey>) -> SchemaKey {
        SchemaKey::new(
            self.data_type_name().cloned(),
            self.default_expression().map(|default| default.remove_annotations()),
            self.validation().map(|validation| validation.remove_annotations()),
            self.key_schema().map(|schema_definition| schema_definition.to_schema_key().into()),
            self.entry_schema().map(|schema_definition| schema_definition.to_schema_key().into()),
            details_schema_key.map(|schema_key| schema_key.into()),
        )
    }

    /// Initialize primitive schema.
    fn initialize_primitive_schema<SchemaDetailsT>(
        &self,
        schema: &mut Schema<AnnotatedT>,
        reference: SchemaReference,
        details: &SchemaDetailsT,
        details_namespace: Option<&Namespace>,
        data_kind: DataKind,
    ) -> SchemaReference
    where
        SchemaDetailsT: SchemaDetails<AnnotatedT>,
    {
        let mut primitive_schema: PrimitiveSchema<_> = data_kind.into();
        complete_schema_default_and_validation!(primitive_schema, self, details, details_namespace);
        schema.add_unique(reference, primitive_schema.into())
    }

    /// Initialize list schema.
    fn initialize_list_schema<SchemaDetailsT>(
        &self,
        schema: &mut Schema<AnnotatedT>,
        reference: SchemaReference,
        details: &SchemaDetailsT,
        details_namespace: Option<&Namespace>,
        context: &mut CompletionContext,
    ) -> Result<Option<SchemaReference>, ToscaError<WithAnnotations>>
    where
        AnnotatedT: 'static,
        SchemaDetailsT: SchemaDetails<AnnotatedT>,
    {
        let mut list_schema = ListSchema::default();

        complete_complex_schema_default_and_validation!(list_schema, self, details, details_namespace, context);
        complete_entry_schema!(list_schema, self, details, details_namespace, schema, context);

        Ok(Some(schema.add_unique(reference, list_schema.into())))
    }

    /// Initialize map schema.
    fn initialize_map_schema<SchemaDetailsT>(
        &self,
        schema: &mut Schema<AnnotatedT>,
        reference: SchemaReference,
        details: &SchemaDetailsT,
        details_namespace: Option<&Namespace>,
        context: &mut CompletionContext,
    ) -> Result<Option<SchemaReference>, ToscaError<WithAnnotations>>
    where
        AnnotatedT: 'static,
        SchemaDetailsT: SchemaDetails<AnnotatedT>,
    {
        let mut map_schema = MapSchema::default();

        complete_complex_schema_default_and_validation!(map_schema, self, details, details_namespace, context);
        complete_key_schema!(map_schema, self, details, details_namespace, schema, context);
        complete_entry_schema!(map_schema, self, details, details_namespace, schema, context);

        Ok(Some(schema.add_unique(reference, map_schema.into())))
    }
}
