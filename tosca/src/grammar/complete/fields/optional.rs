/// Complete optional field.
#[macro_export]
macro_rules! complete_optional_field {
    (
        $field:tt,
        $self:expr,
        $parent:expr $(,)?
    ) => {
        if $self.$field.is_none()
            && let Some(parent_field) = &$parent.$field
        {
            $self.$field = Some(parent_field.clone());
            $self.annotations.clone_field_from(stringify!($field), &$parent.annotations);
        }
    };
}

/// Complete optional field to a function's returned value.
#[macro_export]
macro_rules! complete_optional_field_to {
    (
        $field:tt,
        $self:expr,
        $parent:expr,
        $fn:expr $(,)?
    ) => {
        if $self.$field.is_none() {
            $self.$field = $fn();
            $self.annotations.clone_field_from(stringify!($field), &$parent.annotations);
        }
    };
}

#[allow(unused_imports)]
pub use {complete_optional_field, complete_optional_field_to};
