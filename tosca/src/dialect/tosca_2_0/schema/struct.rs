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
        details: &SchemaDetailsT,
        details_namespace: Option<&Namespace>,
        context: &mut CompletionContext,
    ) -> Result<SchemaReference, ToscaError<WithAnnotations>>
    where
        AnnotatedT: 'static,
        SchemaDetailsT: SchemaDetails<AnnotatedT>,
    {
        let mut struct_schema = StructSchema::default();

        complete_schema_default_and_validation!(struct_schema, self, details, details_namespace);

        if let Some(properties) = &self.properties {
            for (name, property) in properties {
                if let Some(field) = property.initialize_struct_field_schema(schema, context)? {
                    struct_schema.fields.insert(name.clone(), field);
                }
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
        context: &mut CompletionContext,
    ) -> Result<Option<StructSchemaField>, ToscaError<WithAnnotations>>
    where
        AnnotatedT: 'static,
    {
        let (data_type, data_type_namespace) =
            completed_entity_from_full_name_field!(DATA_TYPE, DataType, self, type_name, context);

        let data_type = data_type.to_namespace(data_type_namespace);

        Ok(match data_type {
            Some(data_type) => data_type
                .initialize_schema(schema, self, None, context)?
                .map(|reference| StructSchemaField::new(reference, self.required.unwrap_or(true))),

            None => None,
        })
    }
}
