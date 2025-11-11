use super::{super::super::super::super::grammar::*, call::*};

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
            Expression::Call(call) => call.function.name.0 == function,
            _ => false,
        }
    }

    // /// True if an implicit call.
    // pub fn is_implicit_call(&self, function: &str) -> bool {
    //     match self {
    //         Expression::Call(call) => call.implicit && (call.function.name.0 == function),
    //         _ => false,
    //     }
    // }

    // /// True if a non-internal implicit call.
    // pub fn is_implicit_non_internal_call(&self) -> bool {
    //     match self {
    //         Expression::Call(call) => call.implicit && !call.function.name.0.starts_with('_'),
    //         _ => true,
    //     }
    // }

    /// If it's a call then make it eager.
    pub fn into_eager(self) -> Self {
        if let Expression::Call(mut call) = self {
            call.make_eager();
            call.into()
        } else {
            self
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

    /// Embed the expression in a call call.
    pub fn embed(self, function: &'static str, internal: bool, call_kind: floria::CallKind) -> Self
    where
        AnnotatedT: Annotated + Clone + Default,
    {
        let annotations = self.annotations().cloned();
        Call::new_implicit(function, internal, vec![self], call_kind).with_annotations_option(annotations).into()
    }

    /// Embed after another expression in a call.
    pub fn embed_after(self, other: Self, function: &'static str, internal: bool, call_kind: floria::CallKind) -> Self
    where
        AnnotatedT: Annotated + Clone + Default,
    {
        let annotations = other.annotations().cloned();
        Call::new_implicit(function, internal, vec![other, self], call_kind).with_annotations_option(annotations).into()
    }

    /// Make the expression lazy and embed in `$_assert` if necessary.
    pub fn lazy_assert(self) -> Self
    where
        AnnotatedT: Annotated + Clone + Default,
    {
        if true { self.embed("assert", true, floria::CallKind::Lazy) } else { self.into_lazy() }
    }
}

impl<AnnotatedT> RemoveAnnotations<Expression<WithoutAnnotations>> for &Expression<AnnotatedT>
where
    AnnotatedT: Clone,
{
    fn remove_annotations(self) -> Expression<WithoutAnnotations> {
        match self {
            Expression::Simple(simple) => Expression::Simple(simple.clone().remove_annotations()),
            Expression::List(list) => {
                Expression::List(list.iter().map(|item| item.clone().remove_annotations()).collect())
            }
            Expression::Map(map) => Expression::Map(
                map.iter()
                    .map(|(key, value)| (key.clone().remove_annotations(), value.clone().remove_annotations()))
                    .collect(),
            ),
            Expression::Call(call) => Expression::Call(call.remove_annotations()),
        }
    }
}

impl<AnnotatedT> ToNamespace<Self> for Expression<AnnotatedT>
where
    AnnotatedT: Clone + Default,
{
    fn to_namespace(&self, namespace: Option<&Namespace>) -> Self {
        match self {
            Self::Simple(simple) => Self::Simple(simple.clone()),
            Self::List(list) => Expression::List(list.iter().map(|item| item.to_namespace(namespace)).collect()),
            Self::Map(map) => Expression::Map(
                map.iter().map(|(key, value)| (key.to_namespace(namespace), value.to_namespace(namespace))).collect(),
            ),
            Self::Call(call) => Self::Call(call.to_namespace(namespace)),
        }
    }
}
