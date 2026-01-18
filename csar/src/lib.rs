// https://stackoverflow.com/a/61417700
#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs)]

/*!
Tools for working with
[CSAR](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html#cloud-service-archive-csar-format)
(Cloud Service Archive).

For documentation and usage examples see the
[Puccini site](https://puccini.cloud).
*/

mod errors;

/// CSAR creator.
#[cfg(feature = "creator")]
pub mod creator;

/// pyo3 support.
#[cfg(feature = "pyo3")]
pub mod pyo3;

/// TOSCA meta.
pub mod tosca_meta;

/// CSAR URL.
#[cfg(feature = "url")]
pub mod url;

#[allow(unused_imports)]
pub use {errors::*, tosca_meta::*};
