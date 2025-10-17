use super::descriptor::*;

//
// TypeEntity
//

/// Type entity.
pub trait TypeEntity<AnnotatedT> {
    /// Descriptor.
    fn descriptor(&self) -> TypeEntityDescriptor<'_, AnnotatedT>;
}

/// Implement [TypeEntity].
#[macro_export]
macro_rules! impl_type_entity {
    ( $type:ident $(,)? ) => {
        impl<AnnotatedT> $crate::grammar::TypeEntity<AnnotatedT> for $type<AnnotatedT>
        where
            AnnotatedT: 'static + ::compris::annotate::Annotated + Clone + Default,
        {
            fn descriptor(&self) -> $crate::grammar::TypeEntityDescriptor<'_, AnnotatedT> {
                $crate::grammar::TypeEntityDescriptor {
                    version: self.version.as_ref().map(|version| version.to_string().into()),
                    description: self.description.as_ref(),
                    metadata: &self.metadata,
                    parent: self.derived_from.as_ref(),
                }
            }
        }
    };
}

#[allow(unused_imports)]
pub use impl_type_entity;
