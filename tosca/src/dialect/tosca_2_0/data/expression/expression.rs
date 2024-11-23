use super::call::*;

use {
    compris::{annotate::*, normal::*},
    std::collections::*,
};

//
// Expression
//

/// Expression.
#[derive(Clone, Debug)]
pub enum Expression<AnnotatedT> {
    /// Simple (not a list or a map).
    Simple(Variant<AnnotatedT>),

    /// List.
    List(Vec<Expression<AnnotatedT>>),

    /// Map.
    Map(BTreeMap<Expression<AnnotatedT>, Expression<AnnotatedT>>),

    /// Call.
    Call(Call<AnnotatedT>),
}

impl<AnnotatedT> Expression<AnnotatedT> {
    /// True if a call.
    pub fn is_call(&self, function: &str) -> bool {
        match self {
            Expression::Call(call) => call.function.inner == function,
            _ => false,
        }
    }

    /// True if a call in dialect's plugin.
    pub fn is_native_call(&self, function: &str) -> bool {
        match self {
            Expression::Call(call) => call.is_native() && (call.function.inner == function),
            _ => false,
        }
    }

    /// True if a non-internal call in dialect's plugin.
    pub fn is_standard_call(&self) -> bool {
        match self {
            Expression::Call(call) => call.is_native() && !call.function.as_str().starts_with('_'),
            _ => true,
        }
    }

    /// If it's a call then make it lazy.
    pub fn into_lazy(self) -> Self {
        if let Expression::Call(mut call) = self {
            call.make_lazy();
            call.into()
        } else {
            self
        }
    }

    /// Embed the expression in a call.
    pub fn embed(self, function: &'static str, call_kind: floria::CallKind) -> Self
    where
        AnnotatedT: Annotated + Clone + Default,
    {
        Call::new_native(Text::from(function).with_annotations_from(&self), vec![self], call_kind).into()
    }

    /// Embed after another expression in a call.
    pub fn embed_after(self, other: Self, function: &'static str, call_kind: floria::CallKind) -> Self
    where
        AnnotatedT: Annotated + Clone + Default,
    {
        Call::new_native(Text::from(function).with_annotations_from(&other), vec![other, self], call_kind).into()
    }

    /// Make the expression lazy and embed in `$_assert` if necessary.
    pub fn lazy_assert(self) -> Self
    where
        AnnotatedT: Annotated + Clone + Default,
    {
        if self.is_standard_call() { self.embed("_assert", floria::CallKind::Lazy) } else { self.into_lazy() }
    }
}
