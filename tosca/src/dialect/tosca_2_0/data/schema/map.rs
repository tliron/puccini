use super::{
    super::{data_kind::*, expression::*},
    value::*,
};

use {kutil::cli::depict::*, std::collections::*};

//
// MapSchema
//

/// Map schema.
#[derive(Clone, Debug, Default, Depict)]
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

impl<AnnotatedT> PartialEq for MapSchema<AnnotatedT> {
    fn eq(&self, other: &Self) -> bool {
        (self.key == other.key)
            && (self.entry == other.entry)
            && (self.default == other.default)
            && (self.validation == other.validation)
    }
}

impl<AnnotatedT> Into<Expression<AnnotatedT>> for MapSchema<AnnotatedT>
where
    AnnotatedT: Default,
{
    fn into(self) -> Expression<AnnotatedT> {
        let mut map = BTreeMap::default();

        map.insert("kind".into(), DataKind::Map.as_str().into());

        if let Some(key) = self.key {
            map.insert("key".into(), (key as u64).into());
        }

        if let Some(entry) = self.entry {
            map.insert("entry".into(), (entry as u64).into());
        }

        if let Some(default) = self.default {
            map.insert("default".into(), default);
        }

        if let Some(validation) = self.validation {
            map.insert("validation".into(), validation.into_lazy());
        }

        map.into()
    }
}
