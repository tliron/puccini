use super::{
    super::super::{super::super::grammar::*, data::*, entities::*},
    details::*,
};

use compris::annotate::*;

// TODO: not used?

impl<AnnotatedT> SchemaDetails<AnnotatedT> for SchemaDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn data_type_name(&self) -> Option<&FullName> {
        Some(&self.type_name)
    }

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
