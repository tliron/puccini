use super::{
    super::{data_kind::*, expression::*},
    reference::*,
};

use {compris::annotate::*, kutil::cli::depict::*, std::collections::*};

//
// MapSchema
//

/// Map schema.
#[derive(Clone, Debug, Default, Depict, Eq)]
pub struct MapSchema<AnnotatedT> {
    /// Key schema reference.
    #[depict(option, style(number))]
    pub key: Option<SchemaReference>,

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

impl<AnnotatedT> MapSchema<AnnotatedT> {
    /// Update reference.
    pub fn update_reference(&mut self, old: SchemaReference, new: SchemaReference) {
        if let Some(key) = self.key.as_mut()
            && *key == old
        {
            *key = new;
        }

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

        map.insert("kind".into(), DataKind::Map.as_str().into());

        if let Some(key) = self.key {
            map.insert("key".into(), positions.expression(key));
        }

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

impl<AnnotatedT> PartialEq for MapSchema<AnnotatedT> {
    fn eq(&self, other: &Self) -> bool {
        (self.key == other.key)
            && (self.entry == other.entry)
            && (self.default == other.default)
            && (self.validation == other.validation)
    }
}
