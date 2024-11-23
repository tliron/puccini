use super::schema::*;

use {compris::normal::*, kutil::cli::depict::*};

//
// Schemas
//

/// Schemas.
#[derive(Clone, Debug, Default, Depict)]
pub struct Schemas<AnnotatedT> {
    /// Schemas.
    #[depict(iter(item), as(depict))]
    pub schemas: Vec<Schema<AnnotatedT>>,
}

impl<AnnotatedT> Schemas<AnnotatedT> {
    /// To Compris variant.
    pub fn to_variant(&self) -> Variant<AnnotatedT>
    where
        AnnotatedT: Default,
    {
        self.schemas.iter().map(|schema| schema.to_variant()).collect()
    }
}

//
// 0:
//   kind: Complex
//   properties:
//     string:
//       kind: String
//       default: value
//     integer:
//       kind: Integer
//     float:
//       kind: Float
//       required: false
//     nested:
//       ref: 1
//     size:
//       ref: 2
//     size-list:
//       ref: 3
//
// 1:
//   kind: Complex
//   properties:
//     nested-float:
//       kind: Float
//       validation: {}
//     nested-string:
//       kind: String
//       required: false
//     nested:
//       ref: 0
//
// 2:
//   kind: Scalar
//   scalar-kind: Integer
//   units: {}
//   prefixes: {}
//
// 3:
//   kind: List
//   item:
//     ref: 2
//
