/// Complete `is_empty` field.
#[macro_export]
macro_rules! complete_field {
    (
        $field:tt,
        $self:ident,
        $parent:ident $(,)?
    ) => {
        if $self.$field.is_empty() && !$parent.$field.is_empty() {
            $self.$field = $parent.$field.clone();
            $self.annotations.clone_field_from(stringify!($field), &$parent.annotations);
        }
    };
}

#[allow(unused_imports)]
pub use complete_field;
