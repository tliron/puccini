use super::super::{super::super::grammar::*, data::*, dialect::*, entities::*};

use compris::annotate::*;

impl<AnnotatedT> SchemaDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    /// Initialize a schema.
    pub fn initialize_schema(
        &self,
        schema: &mut Schema<AnnotatedT>,
        context: &mut CompletionContext,
    ) -> Result<Option<SchemaReference>, ToscaError<WithAnnotations>>
    where
        AnnotatedT: 'static,
    {
        let (data_type, data_type_namespace) =
            completed_entity_from_full_name_field!(DATA_TYPE, DataType, self, type_name, context);

        let data_type = data_type.to_namespace(data_type_namespace);

        Ok(match data_type {
            Some(data_type) => data_type.initialize_schema(schema, self, None, context)?,
            None => None,
        })
    }
}
