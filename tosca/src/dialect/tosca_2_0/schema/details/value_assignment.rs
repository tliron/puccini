use super::{
    super::super::{super::super::grammar::*, data::*, entities::*},
    details::*,
};

use compris::annotate::*;

impl<AnnotatedT> SchemaDetails<AnnotatedT> for ValueAssignment<AnnotatedT>
where
    AnnotatedT: Annotated + Clone + Default,
{
    fn data_type_name(&self) -> Option<&FullName> {
        self.type_name.as_ref()
    }

    fn default_expression(&self) -> Option<&Expression<AnnotatedT>> {
        None
    }

    fn key_schema(&self) -> Option<&SchemaDefinition<AnnotatedT>> {
        None
    }

    fn entry_schema(&self) -> Option<&SchemaDefinition<AnnotatedT>> {
        None
    }

    fn validation(&self) -> Option<&Expression<AnnotatedT>> {
        self.validation.as_ref()
    }
}
