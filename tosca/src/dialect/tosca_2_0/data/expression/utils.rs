use super::expression::*;

use {compris::annotate::*, kutil::std::immutable::*};

//
// ExpressionUtilities
//

/// Expression utilities.
pub trait ExpressionUtilities<AnnotatedT> {
    /// Joins the expressions with a function.
    ///
    /// If self or other are already the function then will flatten as a single function.
    ///
    /// When always is true will wrap with the function even if there is just one expression.
    fn join(
        &mut self,
        other: Expression<AnnotatedT>,
        function: &'static str,
        call_kind: floria::CallKind,
        always: bool,
    ) where
        AnnotatedT: Annotated + Clone + Default;

    /// Joins the expressions with "_apply"".
    ///
    /// If self or other are already "_apply" then will flatten as a single "_apply".
    ///
    /// If it's just one expression will wrap in "_apply".
    fn join_apply(&mut self, other: Expression<AnnotatedT>)
    where
        AnnotatedT: Annotated + Clone + Default,
    {
        self.join(other, "_apply", floria::CallKind::Eager, true)
    }

    /// Joins the expressions with "and"".
    ///
    /// If self or other are already "and" then will flatten as a single "and".
    fn join_and(&mut self, other: Expression<AnnotatedT>)
    where
        AnnotatedT: Annotated + Clone + Default,
    {
        self.join(other, "and", floria::CallKind::Normal, false)
    }

    /// Complete validation as a flattened `$and`.
    ///
    /// If self or parent are already `$and` then will flatten as a single `$and`.
    fn complete_validation(
        &mut self,
        parent_validation: Option<&Expression<AnnotatedT>>,
        struct_annotations: &mut StructAnnotations,
        parent_struct_annotations: &StructAnnotations,
    ) where
        AnnotatedT: Annotated + Clone + Default;
}

impl<AnnotatedT> ExpressionUtilities<AnnotatedT> for Option<Expression<AnnotatedT>> {
    fn join(&mut self, other: Expression<AnnotatedT>, function: &'static str, call_kind: floria::CallKind, always: bool)
    where
        AnnotatedT: Annotated + Clone + Default,
    {
        *self = match self.take() {
            Some(expression) => {
                if expression.is_native_call(function) {
                    // Prepend to existing function
                    if let Expression::Call(mut function) = expression {
                        function.prepend_unique_argument(other);
                        Some(function.into())
                    } else {
                        panic!("should be a call");
                    }
                } else if other.is_native_call(function) {
                    // Append to existing function
                    if let Expression::Call(mut function) = other {
                        function.append_unique_argument(expression);
                        Some(function.into())
                    } else {
                        panic!("should be a call");
                    }
                } else {
                    // Join with function
                    Some(expression.embed_after(other, function, call_kind))
                }
            }

            None => {
                if !always || other.is_native_call(function) {
                    // As is
                    Some(other)
                } else {
                    // Embed in function
                    Some(other.embed(function, call_kind))
                }
            }
        };
    }

    fn complete_validation(
        &mut self,
        parent_validation: Option<&Expression<AnnotatedT>>,
        struct_annotations: &mut StructAnnotations,
        parent_struct_annotations: &StructAnnotations,
    ) where
        AnnotatedT: Annotated + Clone + Default,
    {
        if let Some(parent_validation) = parent_validation {
            self.join_and(parent_validation.clone());

            if self.is_none()
                && let Some(annotations) = parent_struct_annotations.get("validation")
            {
                struct_annotations.insert(ByteString::from_static("validation"), annotations.clone());
            }
        }
    }
}

/// Complete validation as a flattened `$and`.
///
/// If self or parent are already `$and` then will flatten as a single `$and`.
#[macro_export]
macro_rules! complete_validation (
    (
        $self:ident,
        $parent:ident $(,)?
    ) => {
        $self.validation.complete_validation(
            $parent.validation.as_ref(),
            &mut $self.annotations,
            &$parent.annotations,
        )
    }
);

#[allow(unused_imports)]
pub use complete_validation;
