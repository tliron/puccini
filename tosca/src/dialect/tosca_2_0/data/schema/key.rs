use super::super::{super::super::super::grammar::*, expression::*};

use {compris::annotate::*, depiction::*};

//
// SchemaKey
//

/// [Schema](super::Schema) key.
///
/// Comprises the minimal set of factors that determine a schema.
#[derive(Clone, Debug, Depict, Eq, Hash, PartialEq)]
pub struct SchemaKey {
    /// [DataType](super::super::super::DataType) name.
    #[depict(option)]
    pub data_type: Option<FullName>,

    /// Default.
    #[depict(option, as(depict))]
    pub default: Option<Expression<WithoutAnnotations>>,

    /// Validation.
    #[depict(option, as(depict))]
    pub validation: Option<Expression<WithoutAnnotations>>,

    /// Key schema.
    #[depict(option, as(depict))]
    pub key_schema: Option<Box<SchemaKey>>,

    /// Entry schema.
    #[depict(option, as(depict))]
    pub entry_schema: Option<Box<SchemaKey>>,

    /// Schema details.
    #[depict(option, as(depict))]
    pub details: Option<Box<SchemaKey>>,
}

impl SchemaKey {
    /// Constructor.
    pub fn new(
        data_type: Option<FullName>,
        default: Option<Expression<WithoutAnnotations>>,
        validation: Option<Expression<WithoutAnnotations>>,
        key_schema: Option<Box<SchemaKey>>,
        entry_schema: Option<Box<SchemaKey>>,
        details: Option<Box<SchemaKey>>,
    ) -> Self {
        Self { data_type, default, validation, key_schema, entry_schema, details }
    }
}
