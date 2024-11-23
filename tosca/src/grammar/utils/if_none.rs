use {compris::annotate::*, kutil::std::immutable::*};

/// Clone [StructAnnotations].
pub fn clone_struct_annotations(
    struct_annotations: &StructAnnotations,
    field_names: &[&'static str],
) -> StructAnnotations {
    let mut clone = StructAnnotations::default();
    for field_name in field_names {
        if let Some(annotations) = struct_annotations.get(*field_name) {
            clone.insert(ByteString::from_static(*field_name), annotations.clone());
        }
    }
    clone
}

/// Clone if [None].
#[macro_export]
macro_rules! if_none_clone {
    (
        $field:tt,
        $self:ident,
        $entity:ident $(,)?
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

/// Clone if `is_empty`.
#[macro_export]
macro_rules! if_empty_clone {
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

/// Set to an expression if [None].
#[macro_export]
macro_rules! if_none_else {
    (
        $field:tt,
        $self:ident,
        $entity:ident,
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

#[allow(unused_imports)]
pub use {if_empty_clone, if_none_clone, if_none_else};
