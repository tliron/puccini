// https://stackoverflow.com/a/61417700
#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs)]

/*!
Kubernetes resource model functions.
*/

/// Kubernetes client.
pub mod client;

/// Functions.
pub mod functions;

/// Dispatch.
pub mod dispatch;
