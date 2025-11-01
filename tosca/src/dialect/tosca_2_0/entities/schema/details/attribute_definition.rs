use super::{
    super::super::{super::data::*, attribute_definition::*, schema_definition::*},
    details::*,
};

use compris::annotate::*;

impl<AnnotatedT> SchemaDetails<AnnotatedT> for AttributeDefinition<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn default_expression(&self) -> Option<&Expression<AnnotatedT>> {
        self.default.as_ref()
    }

    fn key_schema(&self) -> Option<&SchemaDefinition<AnnotatedT>> {
        self.key_schema.as_ref()
    }

    fn entry_schema(&self) -> Option<&SchemaDefinition<AnnotatedT>> {
        self.entry_schema.as_ref()
    }

    fn validation(&self) -> Option<&Expression<AnnotatedT>> {
        self.validation.as_ref()
    }
}
