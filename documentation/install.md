[⇐ to main site](https://puccini.cloud)

Installation Guide
==================

Download the Puccini Tools
--------------------------

| This section is a placeholder. We are currently *not* publishing a distribution in this early phase of development. For now, skip below to [building from source](#building-puccini-from-source).

Check out our [published distributions](https://github.com/tliron/puccini/releases).

Included are [`puccini-tosca`](puccini-tosca), [`puccini-csar`](puccini-csar), the examples, and this documentation. All binaries are optimized builds with all the default features enabled.

| Note that the `puccini-tosca` binary internally bundles a precompiled version of the TOSCA 2.0 Wasm plugin. If you need the originally Wasm then you can build it from source (see below).

Building Puccini from Source
----------------------------

If you need a build for other architectures or targets, with debug information, or with features other than the defaults, then you can build it yourself.

| We assume Linux for the build environment. Note that you can crosscompile from Linux to all supported architectures and targets.

### Prerequisites

You need the [Rust](https://rust-lang.org) compiler and standard library as well as its [Cargo](https://doc.rust-lang.org/cargo/) build system. You can get everything at once via [rustup](https://rustup.rs). Note that by default built binaries will be located in `$HOME/.cargo/bin/`, so you may want to add it to your execution `PATH`.

We'll also need the WASI (preview 2) platform for building our Wasm plugins. We recommend installing the [Wild linker](https://github.com/davidlattimore/wild) (only available for Linux), which Puccini can use for faster building of debug builds:

```
rustup target add wasm32-wasip2
rustup +nightly target add wasm32-wasip2
cargo install --locked wild-linker
```

### Build!

```
git clone https://github.com/tliron/puccini.git
cd puccini

# You may want to checkout a specific release tag, for example:
git checkout v0.0.3

# Build!
scripts/build

# Verify it
~/.cargo/bin/puccini-tosca version --build
```

The default for `scripts/build` is a debug build:

* builds faster
* slow runtime performance: *no* optimizations enabled
* includes debug information (larger binaries)

Unless you're developing Puccini, you'll want a release:

* very slow to build, beware!
* best runtime performance: *all* optimizations enabled

```
scripts/build -r

# Verify it
~/.cargo/bin/puccini-tosca version --build
```

The build script can be configured with several environment variables. See its source for documentation. Example:

```
WASM_PRECOMPILE=true WASM_DEBUG_INFO=false scripts/build
```

<!--
```
cargo install puccini-cli --debug
cargo install puccini-cli --no-default-features --features=tosca-2_0,filesystem
```
-->
