// TODO: Can we avoid cloning? The problem is that we need to later refer to catalog

/// Get completed [Entity](super::Entity) and [Scope](super::super::name::Scope) from
/// [FullName](super::super::name::FullName) field.
#[macro_export]
macro_rules! entity_from_name_field {
    (
        $kind:ident,
        $tosca_type:ident,
        $self:ident,
        $field:tt,
        $catalog:ident,
        $source_id:ident,
        $errors:ident $(,)?
    ) => {
        match $catalog
            .completed_entity::<$tosca_type<AnnotatedT>, _, _>(
                $kind,
                &$self.$field,
                $source_id,
                &mut $errors.with_fallback_annotations_from_field($self, stringify!($field)),
            )?
            .cloned()
        {
            Some(entity) => (Some(entity), Some(&$self.$field.scope)),
            None => (None, Some(&$self.$field.scope)),
        }
    };
}

/// Get completed [Entity](super::Entity) and [Scope](super::super::name::Scope) from
/// optional [FullName](super::super::name::FullName) field.
#[macro_export]
macro_rules! entity_from_name_field_option {
    (
        $kind:ident,
        $tosca_type:ident,
        $self:ident,
        $field:tt,
        $catalog:ident,
        $source_id:ident,
        $errors:ident $(,)?
    ) => {
        match &$self.$field {
            Some(full_name) => match $catalog
                .completed_entity::<$tosca_type<AnnotatedT>, _, _>(
                    $kind,
                    full_name,
                    $source_id,
                    &mut $errors.with_fallback_annotations_from_field($self, stringify!($field)),
                )?
                .cloned()
            {
                Some(entity) => (Some(entity), Some(&full_name.scope)),
                None => (None, None),
            },

            None => (None, None),
        }
    };
}

/// Get completed [Entity](super::Entity) and [Scope](super::super::name::Scope) from
/// [FullName](super::super::name::FullName) field.
#[macro_export]
macro_rules! entity_from_name_field_checked {
    (
        $kind:ident,
        $self:ident,
        $field:tt,
        $catalog:ident,
        $source_id:ident,
        $derivation_path:ident,
        $errors:ident $(,)?
    ) => {
        match &$self.$field {
            Some(full_name) => {
                match $catalog
                    .completed_entity_checked::<Self, _, _>(
                        $kind,
                        full_name,
                        $source_id,
                        $derivation_path,
                        &mut $errors.with_fallback_annotations_from_field($self, stringify!($field)),
                    )?
                    .cloned()
                {
                    Some(parent) => (Some(parent), Some(&full_name.scope)),
                    None => (None, None),
                }
            }

            None => (None, None),
        }
    };
}

#[allow(unused_imports)]
pub use {entity_from_name_field, entity_from_name_field_checked, entity_from_name_field_option};
