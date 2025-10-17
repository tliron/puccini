use super::namespace::*;

use {kutil::std::immutable::*, std::collections::*};

//
// ToNamespace
//

/// Clone or convert into an entity in an optional [Namespace].
pub trait ToNamespace<IntoT> {
    /// Clone or convert into an entity in an optional [Namespace].
    fn to_namespace(&self, namespace: Option<&Namespace>) -> IntoT;
}

impl<IntoNamespaceT> ToNamespace<Self> for Option<IntoNamespaceT>
where
    IntoNamespaceT: Clone + ToNamespace<IntoNamespaceT>,
{
    fn to_namespace(&self, namespace: Option<&Namespace>) -> Self {
        let mut clone = self.clone();
        if namespace.is_some() {
            clone = clone.map(|clone| clone.to_namespace(namespace));
        }
        clone
    }
}

impl<IntoNamespaceT> ToNamespace<Self> for Box<IntoNamespaceT>
where
    IntoNamespaceT: Clone + ToNamespace<IntoNamespaceT>,
{
    fn to_namespace(&self, namespace: Option<&Namespace>) -> Self {
        let mut clone = self.clone();
        if namespace.is_some() {
            clone = Box::leak(clone).to_namespace(namespace).into();
        }
        clone
    }
}

impl<IntoNamespaceT, IntoT> ToNamespace<Vec<IntoT>> for Vec<IntoNamespaceT>
where
    IntoNamespaceT: ToNamespace<IntoT>,
{
    fn to_namespace(&self, namespace: Option<&Namespace>) -> Vec<IntoT> {
        self.iter().map(|entry| entry.to_namespace(namespace)).collect()
    }
}

impl<IntoNamespaceT, IntoT> ToNamespace<BTreeMap<ByteString, IntoT>> for BTreeMap<ByteString, IntoNamespaceT>
where
    IntoNamespaceT: ToNamespace<IntoT>,
{
    fn to_namespace(&self, namespace: Option<&Namespace>) -> BTreeMap<ByteString, IntoT> {
        self.iter().map(|(name, value)| (name.clone(), value.to_namespace(namespace))).collect()
    }
}
