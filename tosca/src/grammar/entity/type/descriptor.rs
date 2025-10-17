use super::super::super::{data::*, name::*};

use kutil::std::immutable::*;

//
// TypeEntityDescriptor
//

/// Type entity descriptor.
pub struct TypeEntityDescriptor<'own, AnnotatedT> {
    /// Version.
    pub version: Option<ByteString>,

    /// Description.
    pub description: Option<&'own ByteString>,

    /// Metadata.
    pub metadata: &'own Metadata<AnnotatedT>,

    /// Parent.
    pub parent: Option<&'own FullName>,
}
