use super::scope::*;

use {kutil::std::immutable::*, std::collections::*};

//
// IntoScoped
//

/// Clone or convert, optionally in a [Scope].
pub trait IntoScoped<IntoT> {
    /// Clone or convert, optionally in a [Scope].
    fn into_scoped(&self, scope: Option<&Scope>) -> IntoT;
}

impl<IntoScopedT> IntoScoped<Option<IntoScopedT>> for Option<IntoScopedT>
where
    IntoScopedT: Clone + IntoScoped<IntoScopedT>,
{
    fn into_scoped(&self, scope: Option<&Scope>) -> Option<IntoScopedT> {
        let mut into_scoped = self.clone();
        if scope.is_some() {
            into_scoped = into_scoped.map(|into_scoped| into_scoped.into_scoped(scope));
        }
        into_scoped
    }
}

impl<IntoScopedT> IntoScoped<Vec<IntoScopedT>> for Vec<IntoScopedT>
where
    IntoScopedT: IntoScoped<IntoScopedT>,
{
    fn into_scoped(&self, scope: Option<&Scope>) -> Vec<IntoScopedT> {
        self.iter().map(|into_scoped| into_scoped.into_scoped(scope)).collect()
    }
}

impl<IntoScopedT, IntoT> IntoScoped<BTreeMap<ByteString, IntoT>> for BTreeMap<ByteString, IntoScopedT>
where
    IntoScopedT: IntoScoped<IntoT>,
{
    fn into_scoped(&self, scope: Option<&Scope>) -> BTreeMap<ByteString, IntoT> {
        self.iter().map(|(name, into_scoped)| (name.clone(), into_scoped.into_scoped(scope))).collect()
    }
}
