/// Complete field of tagged values of [Subentity](super::super::Subentity).
#[macro_export]
macro_rules! complete_subentity_tagged_values_field {
    (
        $type_name:tt,
        $field:tt,
        $self:expr,
        $scope:expr,
        $entity:expr,
        $must_be_declared:expr,
        $catalog:expr,
        $source_id:expr,
        $errors:ident $(,)?
    ) => {
        errors_with_fallback_annotations_from_field!(
            $errors, $self, stringify!($field),
            complete_subentity_tagged_values(
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

#[allow(unused_imports)]
pub use complete_subentity_tagged_values_field;
