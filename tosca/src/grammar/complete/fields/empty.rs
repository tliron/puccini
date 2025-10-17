/// Complete `is_empty` field.
#[macro_export]
macro_rules! complete_empty_field {
    (
        $field:tt,
        $self:ident,
        $entity:ident $(,)?
    ) => {
        if $self.$field.is_empty() && !$entity.$field.is_empty() {
            $self.$field = $entity.$field.clone();
            $self.annotations.clone_field_from(stringify!($field), &$entity.annotations);
        }
    };
}

#[allow(unused_imports)]
pub use complete_empty_field;
