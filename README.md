*Work in progress, not ready for general use. For now, see the [previous version](https://github.com/tliron/go-puccini).*

[![crates.io](https://img.shields.io/crates/v/puccini-tosca?color=%23227700)](https://crates.io/crates/puccini-tosca)
[![docs.rs](https://img.shields.io/badge/docs.rs-latest?color=grey)](https://docs.rs/puccini-tosca/latest/puccini_tosca/)

Puccini
=======

Tools for working with [TOSCA](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html) (Topology and Orchestration Specification for Cloud Applications) and [CSAR](https://docs.oasis-open.org/tosca/TOSCA/v2.0/TOSCA-v2.0.html#cloud-service-archive-csar-format) (Cloud Service Archive).

It comprises:

* Libraries for TOSCA (parsing, validation, compilation) and CSAR (validation, reading, writing)
* [Wasm](https://webassembly.org) implementations for TOSCA's built-in functions
* CLI tools based on the above:

`puccini-tosca`
---------------

TOSCA parser, validator, and compiler.

The compilation output is [Floria](https://floria.khutulun.org) templates, which can then be instantiated by Puccini in order to test imperative behaviors such as calling TOSCA functions embedded in properties, attributes, and outputs, as well as invoking operations and responding to notifications.

For a Floria-based cloud orchestrator, see [Khutulun](https://khutulun.org).

`puccini-csar`
--------------

Can create compliant CSAR files, validate existing ones, extract contained artifacts, and query or modify the metadata.

Supports both tarballs and legacy ZIP files.

Documentation
-------------

* [Installation Guide](https://puccini.cloud/documentation/install)
* [puccini-tosca Guide](https://puccini.cloud/documentation/puccini-tosca)
* [puccini-csar Guide](https://puccini.cloud/documentation/puccini-csar)
* [Frequently Asked Questions](https://puccini.cloud/documentation/faq)
* [Puccini and Floria](https://puccini.cloud/documentation/floria)
* [puccini-tosca API Documentation](https://docs.rs/puccini-tosca/latest/puccini_tosca/),
  [examples](https://github.com/tliron/puccini/tree/main/tosca/examples)
* [puccini-csar API Documentation](https://docs.rs/puccini-csar/latest/puccini_csar/),
  [examples](https://github.com/tliron/puccini/tree/main/csar/examples)

License
-------

Like much of the Rust ecosystem, licensed under your choice of either of

* [Apache License, Version 2.0](https://github.com/tliron/puccini/blob/main/LICENSE-APACHE)
* [MIT license](https://github.com/tliron/puccini/blob/main/LICENSE-MIT)

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
