use super::{super::expression::*, kind::*};

use {compris::normal::*, kutil::cli::depict::*};

//
// Schema
//

/// Schema.
#[derive(Clone, Debug, Depict)]
pub struct Schema<AnnotatedT> {
    /// Kind.
    #[depict(as(depict))]
    kind: SchemaKind<AnnotatedT>,

    /// Default.
    #[depict(option, as(depict))]
    pub default: Option<Expression<AnnotatedT>>,

    /// Validator.
    #[depict(option, as(depict))]
    pub validator: Option<Expression<AnnotatedT>>,
}

impl<AnnotatedT> Schema<AnnotatedT> {
    /// To Compris variant.
    pub fn to_variant(&self) -> Variant<AnnotatedT>
    where
        AnnotatedT: Default,
    {
        let mut map = Map::default();

        self.kind.to_variant(&mut map);

        // if let Some(default) = &self.default {
        //     map.into_insert("default", default.into());
        // }

        map.into()
    }
}
