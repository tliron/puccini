/// Errors with fallback annotations from field.
#[macro_export]
macro_rules! errors_with_fallback_annotations_from_field (
    ( $errors:ident, $self:ident, $field:expr, $( $code:tt )* ) => {
        {
            let annotations = $self.field_annotations($field).cloned();
            let $errors = &mut $errors.with_fallback_annotations(annotations.as_ref());
            $( $code )*
        }
    };
);

#[allow(unused_imports)]
pub use errors_with_fallback_annotations_from_field;
