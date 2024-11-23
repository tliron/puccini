use super::{
    super::{data_kind::*, expression::*},
    reference::*,
};

use {compris::annotate::*, kutil::cli::depict::*, std::collections::*};

//
// ListSchema
//

/// List schema.
#[derive(Clone, Debug, Default, Depict, Eq)]
pub struct ListSchema<AnnotatedT> {
    /// Entry schema reference.
    #[depict(option, style(number))]
    pub entry: Option<SchemaReference>,

    /// Default.
    #[depict(option, as(depict))]
    pub default: Option<Expression<AnnotatedT>>,

    /// Validation.
    #[depict(option, as(depict))]
    pub validation: Option<Expression<AnnotatedT>>,
}

impl<AnnotatedT> ListSchema<AnnotatedT> {
    /// Update reference.
    pub fn update_reference(&mut self, old: SchemaReference, new: SchemaReference) {
        if let Some(entry) = self.entry.as_mut()
            && *entry == old
        {
            *entry = new;
        }
    }

    /// Into expression.
    pub fn into_expression(self, positions: &SchemaReferencePositions) -> Expression<AnnotatedT>
    where
        AnnotatedT: Annotated + Clone + Default,
    {
        let mut map = BTreeMap::default();

        map.insert("kind".into(), DataKind::List.as_str().into());

        if let Some(entry) = self.entry {
            map.insert("entry".into(), positions.expression(entry));
        }

        if let Some(default) = self.default {
            map.insert("default".into(), default);
        }

        if let Some(validation) = self.validation {
            map.insert("validation".into(), validation.lazy_assert());
        }

        map.into()
    }
}

impl<AnnotatedT> PartialEq for ListSchema<AnnotatedT> {
    fn eq(&self, other: &Self) -> bool {
        (self.entry == other.entry) && (self.default == other.default) && (self.validation == other.validation)
    }
}
