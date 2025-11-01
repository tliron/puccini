use super::{
    super::super::{super::data::*, schema_definition::*},
    details::*,
};

use compris::annotate::*;

// TODO: not used!!!!

impl<AnnotatedT> SchemaDetails<AnnotatedT> for SchemaDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn default_expression(&self) -> Option<&Expression<AnnotatedT>> {
        None
    }

    fn key_schema(&self) -> Option<&SchemaDefinition<AnnotatedT>> {
        self.key_schema.as_ref().map(|key_schema| key_schema.as_ref())
    }

    fn entry_schema(&self) -> Option<&SchemaDefinition<AnnotatedT>> {
        self.entry_schema.as_ref().map(|entry_schema| entry_schema.as_ref())
    }

    fn validation(&self) -> Option<&Expression<AnnotatedT>> {
        self.validation.as_ref()
    }
}
