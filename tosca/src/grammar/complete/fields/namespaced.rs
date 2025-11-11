/// Complete namespaced field.
#[macro_export]
macro_rules! complete_namespaced_field {
    (
        $field:tt,
        $self:expr,
        $parent:expr,
        $parent_namespace:expr,
        $context:expr $(,)?
    ) => {
        if $self.$field.is_none()
            && let Some(parent_field) = &$parent.$field
        {
            $self.$field = Some(parent_field.to_namespace($parent_namespace));
            $self.annotations.clone_field_from(stringify!($field), &$parent.annotations);
        }
    };
}

#[allow(unused_imports)]
pub use complete_namespaced_field;
