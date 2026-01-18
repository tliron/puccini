use super::{
    super::{super::super::grammar::*, data::*, entities::*},
    dialect::*,
    entity_kind::*,
};

use {compris::annotate::*, kutil::std::immutable::*, problemo::*};

/// Wasm artifact type.
pub const WASM_ARTIFACT_TYPE: Name = Name::new_static_unchecked("Wasm");

/// Wasm plugin artifact type.
pub const WASM_PLUGIN_ARTIFACT_TYPE: Name = Name::new_static_unchecked("WasmPlugin");

/// Implicit source ID.
pub const IMPLICIT_SOURCE_ID: SourceID = SourceID::Internal(DIALECT_ID);

/// Puccini source ID.
pub const PUCCINI_SOURCE_ID: SourceID = SourceID::Profile(ByteString::from_static("puccini"));

/// Internal namespace.
pub fn internal_namespace() -> Namespace {
    Namespace::from(vec!["_internal".into()])
}

impl super::Dialect {
    /// Create the built-in sources.
    pub fn built_in_sources<AnnotatedT>() -> Result<Vec<Source>, Problem>
    where
        AnnotatedT: 'static + Annotated + Clone + Default,
    {
        let mut implicit = Source::new(IMPLICIT_SOURCE_ID, DIALECT_ID);
        Self::add_implicit_data_types::<AnnotatedT>(&mut implicit)?;
        Self::add_implicit_functions::<AnnotatedT>(&mut implicit)?;

        let mut puccini = Source::new(PUCCINI_SOURCE_ID, DIALECT_ID);
        puccini.merge_namespace(&implicit, &Default::default(), &mut FailFast)?;
        Self::add_puccini_artifact_types::<AnnotatedT>(&mut puccini)?;
        Self::add_puccini_functions::<AnnotatedT>(&mut puccini)?;

        // The implicit functions refer to _internal::WasmPlugin
        implicit.merge_namespace(&puccini, &internal_namespace(), &mut FailFast)?;

        Ok(vec![implicit, puccini])
    }

    fn add_implicit_data_types<AnnotatedT>(source: &mut Source) -> Result<(), Problem>
    where
        AnnotatedT: 'static + Annotated + Clone + Default,
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
            source.add_entity(DATA_TYPE, data_kind.into(), DataType::<AnnotatedT>::new_internal(data_kind), false)?;
        }
        Ok(())
    }

    fn add_implicit_functions<AnnotatedT>(source: &mut Source) -> Result<(), Problem>
    where
        AnnotatedT: 'static + Annotated + Clone + Default,
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
        ] {
            source.add_entity(
                FUNCTION,
                function.into(),
                FunctionDefinition::<AnnotatedT>::new_internal(internal_namespace(), PLUGIN_URL),
                false,
            )?;
        }

        Ok(())
    }

    fn add_puccini_artifact_types<AnnotatedT>(source: &mut Source) -> Result<(), Problem>
    where
        AnnotatedT: 'static + Annotated + Clone + Default,
    {
        let mut wasm = ArtifactType::<AnnotatedT>::new_internal();

        wasm.mime_type = Some("application/wasm".into());
        wasm.file_ext = Some(vec!["wasm".into(), "cwasm".into()]);

        source.add_entity(ARTIFACT_TYPE, WASM_ARTIFACT_TYPE, wasm, true)?;

        let mut wasm_plugin = ArtifactType::<AnnotatedT>::new_internal();

        let mut global = PropertyDefinition::<AnnotatedT>::default();
        global.type_name = Name::new_static_unchecked("boolean").into();
        global.required = Some(false);

        let mut function = PropertyDefinition::<AnnotatedT>::default();
        function.type_name = Name::new_static_unchecked("string").into();
        function.required = Some(false);

        let mut event = PropertyDefinition::<AnnotatedT>::default();
        event.type_name = Name::new_static_unchecked("string").into();
        event.required = Some(false);

        wasm_plugin.derived_from = Some(WASM_ARTIFACT_TYPE.into());
        wasm_plugin.properties.insert("global".into(), global);
        wasm_plugin.properties.insert("function".into(), function);
        wasm_plugin.properties.insert("event".into(), event);

        source.add_entity(ARTIFACT_TYPE, WASM_PLUGIN_ARTIFACT_TYPE, wasm_plugin, false)?;

        Ok(())
    }

    fn add_puccini_functions<AnnotatedT>(source: &mut Source) -> Result<(), Problem>
    where
        AnnotatedT: 'static + Annotated + Clone + Default,
    {
        for function in ["apply", "assert", "schema", "select_capability"] {
            source.add_entity(
                FUNCTION,
                function.into(),
                FunctionDefinition::<AnnotatedT>::new_internal(Default::default(), PLUGIN_URL),
                false,
            )?;
        }

        Ok(())
    }
}
