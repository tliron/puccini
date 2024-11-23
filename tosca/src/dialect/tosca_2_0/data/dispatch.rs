use super::super::dialect::*;

use kutil::std::immutable::*;

/// Dispatch name.
pub fn get_dispatch_name(name: &str) -> ByteString {
    format!("{}:{}", DIALECT_ID, name).into()
}
