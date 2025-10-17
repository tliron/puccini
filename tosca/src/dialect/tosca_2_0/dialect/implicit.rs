use super::{
    super::{super::super::grammar::*, data::*, entities::*},
    dialect::*,
    entity_kind::*,
};

use {compris::annotate::*, kutil::std::immutable::*, std::fmt};

/// Wasm artifact type.
pub const WASM_ARTIFACT_TYPE: &str = "_Wasm";

/// Plugin artifact type.
pub const PLUGIN_ARTIFACT_TYPE: &str = "_Plugin";

impl super::Dialect {
    /// Create the implicit source.
    pub fn implicit_source<AnnotatedT>() -> Source
    where
        AnnotatedT: 'static + Annotated + Clone + fmt::Debug + Default,
    {
        let mut source = Source::new(SourceID::Internal(DIALECT_ID), DIALECT_ID);

        Self::add_data_types::<AnnotatedT>(&mut source);
        Self::add_artifact_types::<AnnotatedT>(&mut source);
        Self::add_functions::<AnnotatedT>(&mut source);

        source
    }

    fn add_data_types<AnnotatedT>(source: &mut Source)
    where
        AnnotatedT: 'static + Annotated + Clone + fmt::Debug + Default,
    {
        for data_kind in [
            DataKind::String,
            DataKind::Integer,
            DataKind::Float,
            DataKind::Boolean,
            DataKind::Bytes,
            DataKind::Nil,
            DataKind::Timestamp,
            DataKind::Scalar,
            DataKind::Version,
            DataKind::List,
            DataKind::Map,
        ] {
            source
                .add_entity::<_, AnnotatedT>(
                    DATA_TYPE,
                    data_kind.into(),
                    DataType::<AnnotatedT>::new_internal(data_kind),
                    true,
                )
                .expect("add_entity");
        }
    }

    fn add_artifact_types<AnnotatedT>(source: &mut Source)
    where
        AnnotatedT: 'static + Annotated + Clone + fmt::Debug + Default,
    {
        let mut wasm = ArtifactType::<AnnotatedT>::new_internal();

        wasm.mime_type = Some("application/wasm".into());
        wasm.file_ext = Some(vec!["wasm".into()]);

        source.add_entity::<_, AnnotatedT>(ARTIFACT_TYPE, WASM_ARTIFACT_TYPE.into(), wasm, true).expect("add_entity");

        let mut plugin = ArtifactType::<AnnotatedT>::new_internal();

        let mut floria_prefix = PropertyDefinition::<AnnotatedT>::default();
        floria_prefix.type_name = Name::from("string").into();

        plugin.derived_from = Some(Name::from(WASM_ARTIFACT_TYPE).into());
        plugin.properties.insert(ByteString::from_static("floria-prefix"), floria_prefix);

        source
            .add_entity::<_, AnnotatedT>(ARTIFACT_TYPE, PLUGIN_ARTIFACT_TYPE.into(), plugin, true)
            .expect("add_entity");
    }

    fn add_functions<AnnotatedT>(source: &mut Source)
    where
        AnnotatedT: 'static + Annotated + Clone + fmt::Debug + Default,
    {
        for function in [
            "ceil",
            "difference",
            "floor",
            "product",
            "quotient",
            "remainder",
            "round",
            "sum",
            "contains",
            "has_all_entries",
            "has_all_keys",
            "has_any_entry",
            "has_any_key",
            "has_entry",
            "has_key",
            "has_prefix",
            "has_suffix",
            "equal",
            "greater_or_equal",
            "greater_than",
            "less_or_equal",
            "less_than",
            "matches",
            "valid_values",
            "and",
            "not",
            "or",
            "xor",
            "concat",
            "join",
            "length",
            "token",
            "available_allocation",
            "get_artifact",
            "get_attribute",
            "get_input",
            "get_property",
            "node_index",
            "relationship_index",
            "value",
            "intersection",
            "union",
            "_apply",
            "_assert",
            "_schema",
            "_select_capability",
        ] {
            source
                .add_entity::<_, AnnotatedT>(
                    FUNCTION,
                    function.into(),
                    FunctionDefinition::<AnnotatedT>::new_internal_plugin(DIALECT_ID),
                    true,
                )
                .expect("add_entity");
        }
    }
}
