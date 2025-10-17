/// Complete optional field.
#[macro_export]
macro_rules! complete_none_field {
    (
        $field:tt,
        $self:expr,
        $entity:expr $(,)?
    ) => {{
        if $self.$field.is_none() && $entity.$field.is_some() {
            $self.$field = $entity.$field.clone();
            $self.annotations.clone_field_from(stringify!($field), &$entity.annotations);
        }
    }};
}

/// Complete optional field to a function's returned value.
#[macro_export]
macro_rules! complete_none_field_to {
    (
        $field:tt,
        $self:expr,
        $entity:expr,
        $fn:expr $(,)?
    ) => {
        if $self.$field.is_none() {
            $self.$field = $fn();
            $self.annotations.clone_field_from(stringify!($field), &$entity.annotations);
        }
    };
}

#[allow(unused_imports)]
pub use {complete_none_field, complete_none_field_to};
