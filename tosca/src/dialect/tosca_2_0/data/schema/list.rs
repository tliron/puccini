use super::{
    super::{data_kind::*, expression::*},
    value::*,
};

use {kutil::cli::depict::*, std::collections::*};

//
// ListSchema
//

/// List schema.
#[derive(Clone, Debug, Default, Depict)]
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

impl<AnnotatedT> PartialEq for ListSchema<AnnotatedT> {
    fn eq(&self, other: &Self) -> bool {
        (self.entry == other.entry) && (self.default == other.default) && (self.validation == other.validation)
    }
}

impl<AnnotatedT> Into<Expression<AnnotatedT>> for ListSchema<AnnotatedT>
where
    AnnotatedT: Default,
{
    fn into(self) -> Expression<AnnotatedT> {
        let mut map = BTreeMap::default();

        map.insert("kind".into(), DataKind::List.as_str().into());

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
