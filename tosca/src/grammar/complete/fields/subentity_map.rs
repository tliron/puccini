/// Complete a field of a map of [Subentity](super::super::super::Subentity).
#[macro_export]
macro_rules! complete_subentity_map_field {
    (
        $type_name:tt,
        $field:tt,
        $self:expr,
        $entity:expr,
        $entity_namespace:expr,
        $must_be_declared:expr,
        $context:expr $(,)?
    ) => {
        errors_with_fallback_annotations_from_field!(errors, $context.errors, $self, stringify!($field), {
            let context = context_with_errors!($context, errors);
            complete_subentity_map(
                stringify!($type_name),
                &mut $self.$field,
                $entity.as_ref().map(|entity| &entity.$field),
                $entity_namespace,
                $must_be_declared,
                context,
            )?;
        });
    };
}

/// Complete an optional field of a map of [Subentity](super::super::super::Subentity).
#[macro_export]
macro_rules! complete_optional_subentity_map_field {
    (
        $type_name:tt,
        $field:tt,
        $self:expr,
        $entity:expr,
        $entity_namespace:expr,
        $must_be_declared:expr,
        $context:ident $(,)?
    ) => {
        if let Some(mut subentity) = $self.$field.take() {
            errors_with_fallback_annotations_from_field!(errors, $context.errors, $self, stringify!($field), {
                let context = context_with_errors!($context, errors);
                complete_subentity_map(
                    stringify!($type_name),
                    &mut subentity,
                    $entity.as_ref().and_then(|entity| entity.$field.as_ref()),
                    $entity_namespace,
                    $must_be_declared,
                    context,
                )?;
            });

            $self.$field = Some(subentity);
        }
    };
}

#[allow(unused_imports)]
pub use {complete_optional_subentity_map_field, complete_subentity_map_field};
