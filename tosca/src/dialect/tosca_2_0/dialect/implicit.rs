use super::{
    super::{super::super::grammar::*, data::*, entities::*},
    entity_kind::*,
};

use {
    compris::{annotate::*, normal::*},
    std::fmt,
};

impl super::Dialect {
    /// Create the implicit source.
    pub fn implicit_source<AnnotatedT>() -> Source
    where
        AnnotatedT: 'static + Annotated + Clone + fmt::Debug + Default,
    {
        let mut source = Source::new(SourceID::Internal(super::DIALECT_ID), super::DIALECT_ID);

        for kind in [
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

            let name = kind.to_string().to_lowercase();
            let value = Call::new(get_dispatch_name("value").into(), Default::default());
            let validation =
                Call::new(get_dispatch_name("_is_a").into(), vec![value.into(), Variant::from(name.clone()).into()]);
            data_type.validation = Some(validation.into());

            data_type.data_kind = Some(kind);

            source
                .add_entity::<_, AnnotatedT>(DATA_TYPE, name.parse().expect("parse name"), data_type)
                .expect("add_entity");
        }

        source
    }
}
