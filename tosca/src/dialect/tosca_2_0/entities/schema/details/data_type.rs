use super::{
    super::super::{super::data::*, data_type::*, schema_definition::*},
    details::*,
};

use compris::annotate::*;

impl<AnnotatedT> SchemaDetails<AnnotatedT> for DataType<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn default_expression(&self) -> Option<&Expression<AnnotatedT>> {
        None
    }

    fn validation(&self) -> Option<&Expression<AnnotatedT>> {
        self.validation.as_ref()
    }

    fn key_schema(&self) -> Option<&SchemaDefinition<AnnotatedT>> {
        self.key_schema.as_ref()
    }

    fn entry_schema(&self) -> Option<&SchemaDefinition<AnnotatedT>> {
        self.entry_schema.as_ref()
    }
}
