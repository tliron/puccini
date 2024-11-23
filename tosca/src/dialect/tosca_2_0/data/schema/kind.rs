use super::{complex::*, scalar::*};

use {
    compris::normal::*,
    kutil::{cli::depict::*, std::immutable::*},
};

//
// SchemaKind
//

/// Schema kind.
#[derive(Clone, Debug, Depict)]
pub enum SchemaKind<AnnotatedT> {
    /// Reference.
    #[depict(style(number))]
    Reference(usize),

    /// Primitive.
    #[depict(style(symbol))]
    Primitive(ByteString),

    /// Complex.
    #[depict(as(depict))]
    Complex(ComplexSchema<AnnotatedT>),

    /// Scalar.
    #[depict(as(depict))]
    Scalar(ScalarSchema<AnnotatedT>),
}

impl<AnnotatedT> SchemaKind<AnnotatedT> {
    /// To Compris variant.
    pub fn to_variant(&self, map: &mut Map<AnnotatedT>)
    where
        AnnotatedT: Default,
    {
        match self {
            Self::Reference(reference) => {
                map.into_insert("reference", *reference);
            }

            Self::Primitive(primitive) => {
                map.into_insert("kind", primitive.clone());
            }

            Self::Scalar(scalar) => {
                map.into_insert("kind", "Scalar");
                scalar.to_variant(map);
            }

            Self::Complex(complex) => {
                map.into_insert("kind", "Complex");
                complex.to_variant(map);
            }
        }
    }
}
