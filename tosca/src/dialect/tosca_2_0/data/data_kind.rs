use kutil::std::*;

//
// DataKind
//

/// Data kind.
#[derive(Debug, Display, Clone, Copy, PartialEq, Eq)]
pub enum DataKind {
    /// String.
    String,

    /// Integer.
    Integer,

    /// Float.
    Float,

    /// Boolean.
    Boolean,

    /// Bytes.
    Bytes,

    /// Nil.
    Nil,

    /// Timestamp.
    Timestamp,

    /// Version.
    Version,

    /// Scalar.
    ///
    /// Allows "data_type", "units", "canonical_unit", and "prefixes".
    Scalar,

    /// List.
    ///
    /// Allows "entry_schema".
    List,

    /// Map.
    ///
    /// Allows "key_schema" and "entry_schema".
    Map,

    /// Complex.
    ///
    /// Allows "properties".
    Complex,
}
