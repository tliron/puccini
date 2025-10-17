/// Complete a field of a type name list.
#[macro_export]
macro_rules! complete_type_list_field {
    (
        $field:tt,
        $self:expr,
        $entity:expr,
        $context:expr $(,)?
    ) => {
        if let Some(entity) = &$entity {
            errors_with_fallback_annotations_from_field!(errors, $context.errors, $self, stringify!($field), {
                let context = context_with_errors!($context, errors);
                complete_type_list(&mut $self.$field, &entity.$field, context)?;
            });
        }
    };
}

#[allow(unused_imports)]
pub use complete_type_list_field;
