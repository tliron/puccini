[⇐ to main site](https://puccini.cloud)

Installation Guide
==================

Download the Puccini Tools
--------------------------

Check out our [published binaries](https://github.com/tliron/puccini/releases) for common architectures. Included are [`puccini-tosca`](puccini-tosca), [`puccini-csar`](puccini-csar), examples, and this documentation.

Note that the `puccini-tosca` binary internally bundles a precompiled version of the TOSCA 2.0 Wasm plugin. If you need the originally Wasm then you can build it from source (see below).

All binaries are optimized builds of the latest public version with all the default features.

Building Puccini from Source
----------------------------

If you need a build for other architectures or targets, with debug information, or with features other than the defaults, then you can build from the latest development snapshot or a specific release tag.

We assume Linux for the build environment. Note that you can crosscompile to all supported architectures and targets.

Example:

```
git clone https://github.com/tliron/puccini.git
cd puccini
# git checkout v0.0.3
scripts/build
```

To just build the Wasm plugins:

```
scripts/build-wasm
```

The build scripts can be configured with several environment variables:

```
WASM_PRECOMPILE=true WASM_DEBUG_INFO=false scripts/build
```

<!--
```
cargo install puccini-cli --debug
cargo install puccini-cli --no-default-features --features=tosca-2_0,filesystem
```
-->
