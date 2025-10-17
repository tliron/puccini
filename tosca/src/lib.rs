// https://stackoverflow.com/a/61417700
#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs)]

/*!
Tools for working with [TOSCA](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html)
(Topology and Orchestration Specification for Cloud Applications).

For documentation and usage examples see the
[Puccini site](https://puccini.cloud).
*/

/// Dialect.
pub mod dialect;

/// Grammar.
pub mod grammar;
