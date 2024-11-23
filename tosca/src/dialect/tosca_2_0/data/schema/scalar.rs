use super::super::data_kind::*;

use {
    compris::normal::*,
    kutil::{cli::depict::*, std::immutable::*},
    std::collections::*,
};

//
// ScalarSchema
//

/// Scalar schema.
#[derive(Clone, Debug, Default, Depict)]
pub struct ScalarSchema<AnnotatedT> {
    /// Data kind.
    #[depict(option, style(symbol))]
    pub data_kind: Option<DataKind>,

    /// Units.
    #[depict(iter(kv), as(depict), key_style(string))]
    pub units: BTreeMap<ByteString, Variant<AnnotatedT>>,

    /// Canonical unit.
    #[depict(option, style(string))]
    pub canonical_unit: Option<ByteString>,

    /// Prefixes.
    #[depict(iter(kv), as(depict), key_style(string))]
    pub prefixes: BTreeMap<ByteString, Variant<AnnotatedT>>,
}

impl<AnnotatedT> ScalarSchema<AnnotatedT> {
    /// Constructor.
    pub fn new(
        data_kind: Option<DataKind>,
        units: BTreeMap<ByteString, Variant<AnnotatedT>>,
        canonical_unit: Option<ByteString>,
        prefixes: BTreeMap<ByteString, Variant<AnnotatedT>>,
    ) -> Self {
        Self { data_kind, units, canonical_unit, prefixes }
    }

    /// To Compris variant.
    pub fn to_variant(&self, _map: &mut Map<AnnotatedT>)
    where
        AnnotatedT: Default,
    {
    }
}
