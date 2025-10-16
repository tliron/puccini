[⇐ to main site](https://puccini.cloud)

Installation Guide
==================

Download Tools
--------------

Check out our [releases](https://github.com/tliron/puccini/releases) for common architectures.

These are release builds of the latest published version with all the default features.

Building Tools from Source
--------------------------

If you need a build with debug information or features other than the default then you can build from the latest development snapshot or a specific release tag:

```
git clone https://github.com/tliron/puccini.git
cd puccini
# git checkout v0.1.0
script/build
```

The build script can be controlled with several environment variables:

```
WASM_PRECOMPILE=true WASM_DEBUG_INFO=false scripts/build
```

<!--
```
cargo install puccini-cli --debug
cargo install puccini-cli --no-default-features --features=tosca-2_0,filesystem
```
-->
