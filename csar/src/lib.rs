// https://stackoverflow.com/a/61417700
#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs)]

/*!
CSAR.
*/

mod errors;

/// CSAR meta.
pub mod meta;

#[allow(unused_imports)]
pub use {errors::*, meta::*};
