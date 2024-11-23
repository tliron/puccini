[⇐ to main site](https://puccini.cloud)

Installation Guide
==================

Download Tools
--------------

Check out our [releases](https://github.com/tliron/puccini/releases) for common architectures.

These are release builds of the latest published version with all the default features.

Building Tools from Source
--------------------------

If you need a build with debug information features other than the default then you can build from the latest published version. Examples:

```
cargo install puccini-cli --debug
cargo install puccini-cli --no-default-features --features=tosca-2_0,filesystem
```

To build the latest development version:

```
git clone https://github.com/tliron/puccini.git
cd puccini
script/build
```
