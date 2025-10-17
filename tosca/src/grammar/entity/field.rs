/// Complete optional field.
#[macro_export]
macro_rules! complete_field_none {
    (
        $field:tt,
        $self:expr,
        $entity:expr $(,)?
    ) => {{
        if $self.$field.is_none() && !$entity.$field.is_some() {
            $self.$field = $entity.$field.clone();
            if let Some(annotations) = $entity.annotations.get(stringify!($field)) {
                $self
                    .annotations
                    .insert(::kutil::std::immutable::ByteString::from_static(stringify!($field)), annotations.clone());
            }
        }
    }};
}

/// Complete optional field to an expression.
#[macro_export]
macro_rules! complete_field_none_to {
    (
        $field:tt,
        $self:expr,
        $entity:expr,
        $( $expression:tt )* $(,)?
    ) => {
        if $self.$field.is_none() {
            $self.$field = $( $expression )*;
            if let Some(annotations) = $entity.annotations.get(stringify!($field)) {
                $self.annotations.insert(::kutil::std::immutable::ByteString::from_static(stringify!($field)), annotations.clone());
            }
        }
    };
}

/// Complete `is_empty` field.
#[macro_export]
macro_rules! complete_field_empty {
    (
        $field:tt,
        $self:ident,
        $entity:ident $(,)?
    ) => {
        if $self.$field.is_empty() && !$entity.$field.is_empty() {
            $self.$field = $entity.$field.clone();
            if let Some(annotations) = $entity.annotations.get(stringify!($field)) {
                $self
                    .annotations
                    .insert(::kutil::std::immutable::ByteString::from_static(stringify!($field)), annotations.clone());
            }
        }
    };
}

#[allow(unused_imports)]
pub use {complete_field_empty, complete_field_none, complete_field_none_to};
