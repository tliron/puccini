use super::{super::data::*, name::*, namespace::*};

use std::collections::*;

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
        self.as_ref().map(|inner| inner.to_namespace(namespace))
    }
}

impl<IntoNamespaceT> ToNamespace<Self> for Box<IntoNamespaceT>
where
    IntoNamespaceT: Clone + ToNamespace<IntoNamespaceT>,
{
    fn to_namespace(&self, namespace: Option<&Namespace>) -> Self {
        self.as_ref().to_namespace(namespace).into()
    }
}

impl<IntoNamespaceT, IntoT> ToNamespace<Vec<IntoT>> for Vec<IntoNamespaceT>
where
    IntoNamespaceT: ToNamespace<IntoT>,
{
    fn to_namespace(&self, namespace: Option<&Namespace>) -> Vec<IntoT> {
        self.iter().map(|item| item.to_namespace(namespace)).collect()
    }
}

impl<IntoNamespaceT, IntoT> ToNamespace<BTreeMap<Name, IntoT>> for BTreeMap<Name, IntoNamespaceT>
where
    IntoNamespaceT: ToNamespace<IntoT>,
{
    fn to_namespace(&self, namespace: Option<&Namespace>) -> BTreeMap<Name, IntoT> {
        self.iter().map(|(key, value)| (key.clone(), value.to_namespace(namespace))).collect()
    }
}

impl<IntoNamespaceT, IntoT> ToNamespace<Taxonomy<Name, IntoT>> for Taxonomy<Name, IntoNamespaceT>
where
    IntoNamespaceT: ToNamespace<IntoT>,
{
    fn to_namespace(&self, namespace: Option<&Namespace>) -> Taxonomy<Name, IntoT> {
        self.iter().map(|(key, value)| (key.clone(), value.to_namespace(namespace))).collect()
    }
}
