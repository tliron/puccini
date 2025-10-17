/// Complete field of map of [Subentity](super::super::Subentity).
#[macro_export]
macro_rules! complete_subentity_map_field {
    (
        $type_name:tt,
        $field:tt,
        $scope:expr,
        $self:expr,
        $entity:expr,
        $must_be_declared:expr,
        $catalog:expr,
        $source_id:expr,
        $errors:ident $(,)?
    ) => {
        errors_with_fallback_annotations_from_field!(
            $errors, $self, stringify!($field),
            complete_subentity_map(
                stringify!($type_name),
                $scope,
                &mut $self.$field,
                $entity.as_ref().map(|entity| &entity.$field),
                $must_be_declared,
                $catalog,
                $source_id,
                $errors,
            )?;
        );
    }
}

/// Complete field of optional map of [Subentity](super::super::Subentity).
#[macro_export]
macro_rules! complete_subentity_map_field_option {
    (
        $type_name:tt,
        $field:tt,
        $scope:expr,
        $self:expr,
        $entity:expr,
        $must_be_declared:expr,
        $catalog:expr,
        $source_id:expr,
        $errors:ident $(,)?
    ) => {
        let mut field = match $self.$field.take() {
            Some(field) => field,
            None => Default::default(),
        };

        errors_with_fallback_annotations_from_field!(
            $errors, $self, stringify!($field),
            complete_subentity_map(
                stringify!($type_name),
                $scope,
                &mut field,
                $entity.as_ref().and_then(|entity| entity.$field.as_ref()),
                $must_be_declared,
                $catalog,
                $source_id,
                $errors,
            )?;
        );

        if !field.is_empty() {
            $self.$field = Some(field);
        }
    }
}

#[allow(unused_imports)]
pub use {complete_subentity_map_field, complete_subentity_map_field_option};
