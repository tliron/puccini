// TODO: Can we avoid cloning? The problem is that we need to later refer to catalog

/// Get a completed [Entity](super::Entity) and [Namespace](super::super::name::Namespace) from
/// a [FullName](super::super::name::FullName) field.
#[macro_export]
macro_rules! completed_entity_from_full_name_field {
    (
        $kind:expr,
        $tosca_type:ident,
        $self:expr,
        $field:tt,
        $context:expr $(,)?
    ) => {{
        match $context.catalog.completed_entity::<$tosca_type<AnnotatedT>, _, _>(
            $kind,
            &$self.$field,
            $context.source_id,
            &mut $context.errors.with_fallback_annotations_from_field($self, stringify!($field)),
        )? {
            Some((entity, _source)) => (Some(entity.clone()), Some(&$self.$field.namespace)),
            None => (None, Some(&$self.$field.namespace)),
        }
    }};
}

/// Get a completed [Entity](super::Entity) and [Namespace](super::super::name::Namespace) from
/// an optional [FullName](super::super::name::FullName) field.
#[macro_export]
macro_rules! completed_entity_from_optional_full_name_field {
    (
        $kind:expr,
        $tosca_type:ident,
        $self:expr,
        $field:tt,
        $context:expr $(,)?
    ) => {
        match &$self.$field {
            Some(full_name) => match $context.catalog.completed_entity::<$tosca_type<AnnotatedT>, _, _>(
                $kind,
                full_name,
                $context.source_id,
                &mut $context.errors.with_fallback_annotations_from_field($self, stringify!($field)),
            )? {
                Some((entity, _source)) => (Some(entity.clone()), Some(&full_name.namespace)),
                None => (None, Some(&full_name.namespace)),
            },

            None => (None, None),
        }
    };
}

/// Get a completed [Entity](super::Entity) and [Namespace](super::super::name::Namespace) from
/// a [FullName](super::super::name::FullName) field.
#[macro_export]
macro_rules! completed_entity_checked_from_full_name_field {
    (
        $kind:expr,
        $self:expr,
        $field:tt,
        $derivation_path:expr,
        $context:ident $(,)?
    ) => {
        match &$self.$field {
            Some(full_name) => {
                match $context.catalog.completed_entity_checked::<Self, _, _>(
                    $kind,
                    full_name,
                    $context.source_id,
                    $derivation_path,
                    &mut $context.errors.with_fallback_annotations_from_field($self, stringify!($field)),
                )? {
                    Some((entity, _source)) => (Some(entity.clone()), Some(&full_name.namespace)),
                    None => (None, Some(&full_name.namespace)),
                }
            }

            None => (None, None),
        }
    };
}

/// Get a completed [Entity](super::Entity) from an optional [Name](super::super::name::Name) field.
#[macro_export]
macro_rules! completed_entity_from_optional_name_field {
    (
        $kind:expr,
        $tosca_type:ident,
        $self:expr,
        $field:tt,
        $context:expr $(,)?
    ) => {
        match &$self.$field {
            Some(name) => $context
                .catalog
                .completed_entity::<$tosca_type<AnnotatedT>, _, _>(
                    $kind,
                    &name.into(),
                    $context.source_id,
                    &mut $context.errors.with_fallback_annotations_from_field($self, stringify!($field)),
                )?
                .map(|(entity, _source)| entity.clone()),

            None => None,
        }
    };
}

#[allow(unused_imports)]
pub use {
    completed_entity_checked_from_full_name_field, completed_entity_from_full_name_field,
    completed_entity_from_optional_full_name_field, completed_entity_from_optional_name_field,
};
