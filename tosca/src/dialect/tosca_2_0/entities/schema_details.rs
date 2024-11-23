use super::{
    super::{super::super::grammar::*, data::*},
    schema_definition::*,
};

use compris::annotate::*;

//
// SchemaDetails
//

macro_rules! set_default (
    (
        $value_schema:ident,
        $self:ident,
        $parent:ident $(,)?
    ) => {
        match ($self.default_expression(), $parent.default_expression()) {
            (None, None) => {}
            (Some(default), None) => $value_schema.default = Some(default.clone()),
            (None, Some(default)) => $value_schema.default = Some(default.clone()),
            (Some(_), Some(_)) => return Err(OverrideProhibitedError::new("default".into()).into()),
        }
    }
);

macro_rules! set_validation (
    (
        $value_schema:ident,
        $self:ident,
        $parent:ident $(,)?
    ) => {
        match ($self.validation(), $parent.validation()) {
            (None, None) => {}
            (Some(validation), None) => {
                let validation = validation.clone().embed_in_assert();
                $value_schema.validation = Some(validation)
            }
            (None, Some(validation)) => {
                let validation = validation.clone().embed_in_assert();
                $value_schema.validation = Some(validation)
            }
            (Some(_), Some(_)) => return Err(OverrideProhibitedError::new("validation".into()).into()),
        }
    }
);

macro_rules! set_schema_reference (
    (
        $value_schema:ident,
        $field:ident,
        $get:ident,
        $self:ident,
        $parent:ident,
        $schema:ident,
        $source_id:ident,
        $catalog:ident $(,)?
    ) => {
        match ($self.$get(), $parent.$get()) {
            (None, None) => {}

            (Some($field), None) => {
                if let Some(reference) = $field.initialize_schema($schema, $source_id, $catalog)? {
                    $value_schema.$field = Some(reference);
                }
            }

            (None, Some($field)) => {
                if let Some(reference) = $field.initialize_schema($schema, $source_id, $catalog)? {
                    $value_schema.$field = Some(reference);
                }
            }

            (Some(_), Some(_)) => return Err(OverrideProhibitedError::new(stringify!($field).into()).into()),
        }
    }
);

/// Schema details.
pub trait SchemaDetails<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    /// Default expression.
    fn default_expression(&self) -> Option<&Expression<AnnotatedT>>;

    /// Validation.
    fn validation(&self) -> Option<&Expression<AnnotatedT>>;

    /// Key schema.
    fn key_schema(&self) -> Option<&SchemaDefinition<AnnotatedT>>;

    /// Entry schema.
    fn entry_schema(&self) -> Option<&SchemaDefinition<AnnotatedT>>;

    /// Initialize primitive schema.
    fn initialize_primitive_schema(
        &self,
        schema: &mut Schema<AnnotatedT>,
        data_kind: DataKind,
    ) -> Result<Option<SchemaReference>, ToscaError<WithAnnotations>>
    where
        AnnotatedT: 'static,
    {
        let mut primitive_schema: PrimitiveSchema<_> = data_kind.into();

        if let Some(default) = self.default_expression() {
            primitive_schema.default = Some(default.clone());
        }

        if let Some(validation) = self.validation() {
            let validation = validation.clone().embed_in_assert();
            primitive_schema.validation = Some(validation);
        }

        let reference = schema.get_or_add_value(primitive_schema.into());
        Ok(Some(reference))
    }

    /// Initialize list schema.
    fn initialize_list_schema<SchemaDetailsT>(
        &self,
        schema: &mut Schema<AnnotatedT>,
        schema_details: &SchemaDetailsT,
        source_id: &SourceID,
        catalog: &Catalog,
    ) -> Result<Option<SchemaReference>, ToscaError<WithAnnotations>>
    where
        AnnotatedT: 'static,
        SchemaDetailsT: SchemaDetails<AnnotatedT>,
    {
        // Placeholder
        let list_reference = schema.add_value(0.into());

        let mut list_schema = ListSchema::default();

        set_default!(list_schema, self, schema_details);
        set_validation!(list_schema, self, schema_details);
        set_schema_reference!(list_schema, entry, entry_schema, self, schema_details, schema, source_id, catalog);

        // Replace placeholder
        schema.values[list_reference] = list_schema.into();
        Ok(Some(list_reference))
    }

    /// Initialize map schema.
    fn initialize_map_schema<SchemaDetailsT>(
        &self,
        schema: &mut Schema<AnnotatedT>,
        schema_details: &SchemaDetailsT,
        source_id: &SourceID,
        catalog: &Catalog,
    ) -> Result<Option<SchemaReference>, ToscaError<WithAnnotations>>
    where
        AnnotatedT: 'static,
        SchemaDetailsT: SchemaDetails<AnnotatedT>,
    {
        // Placeholder
        let map_reference = schema.add_value(0.into());

        let mut map_schema = MapSchema::default();

        set_default!(map_schema, self, schema_details);
        set_validation!(map_schema, self, schema_details);
        set_schema_reference!(map_schema, key, key_schema, self, schema_details, schema, source_id, catalog);
        set_schema_reference!(map_schema, entry, entry_schema, self, schema_details, schema, source_id, catalog);

        // Replace placeholder
        schema.values[map_reference] = map_schema.into();
        Ok(Some(map_reference))
    }
}
