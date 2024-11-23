use super::schema::*;

use {
    compris::normal::*,
    kutil::{cli::depict::*, std::immutable::*},
    std::collections::*,
};

//
// ComplexSchema
//

/// Complex schema.
#[derive(Clone, Debug, Default, Depict)]
pub struct ComplexSchema<AnnotatedT> {
    /// Properties.
    #[depict(iter(kv), as(depict), key_style(string))]
    pub properties: BTreeMap<ByteString, Schema<AnnotatedT>>,
}

impl<AnnotatedT> ComplexSchema<AnnotatedT> {
    /// To Compris variant.
    pub fn to_variant(&self, _map: &mut Map<AnnotatedT>)
    where
        AnnotatedT: Default,
    {
    }
}
