// https://stackoverflow.com/a/61417700
#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs)]

/*!
CSAR.
*/

mod errors;

/// CSAR creator.
#[cfg(feature = "creator")]
pub mod creator;

/// CSAR meta.
pub mod meta;

/// CSAR URL.
#[cfg(feature = "url")]
pub mod url;

#[allow(unused_imports)]
pub use {errors::*, meta::*};
