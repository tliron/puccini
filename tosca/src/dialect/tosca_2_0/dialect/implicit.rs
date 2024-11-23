use super::{
    super::{super::super::grammar::*, data::*, entities::*},
    dialect::*,
    entity_kind::*,
};

use {compris::annotate::*, std::fmt};

impl super::Dialect {
    /// Create the implicit source.
    pub fn implicit_source<AnnotatedT>() -> Source
    where
        AnnotatedT: 'static + Annotated + Clone + fmt::Debug + Default,
    {
        let mut source = Source::new(SourceID::Internal(DIALECT_ID), DIALECT_ID);

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
            let mut data_type = DataType::<AnnotatedT>::default();

            data_type.internal = true;
            data_type.data_kind = Some(data_kind);

            source.add_entity::<_, AnnotatedT>(DATA_TYPE, data_kind.into(), data_type, true).expect("add_entity");
        }

        source
    }
}
