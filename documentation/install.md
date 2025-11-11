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

### Prerequisites

We assume Linux for the build environment. Note that you can crosscompile from Linux to all of Rust's supported architectures and targets.

You need the [Rust](https://rust-lang.org) compiler and standard library as well as its [Cargo](https://doc.rust-lang.org/cargo/) build system. You can get everything at once via [rustup](https://rustup.rs).

We'll also need the [WASI](https://wasi.dev) (preview 2) target libraries for Rust in order to build our Wasm plugins.

Finally, we need either [gcc](https://gcc.gnu.org) or [Clang](https://clang.llvm.org). We recommend also installing the [Wild linker](https://github.com/davidlattimore/wild), which Puccini can detect and use for faster building of debug builds.

To install everything:

```
# Install rustup:
# https://rustup.rs

# Configure PATH (you might want to add this to your ~/.bashrc):
. ~/.cargo/env

# Add WASI target:
rustup target add wasm32-wasip2
rustup +nightly target add wasm32-wasip2

# Install gcc:
#  Fedora world: sudo dnf install gcc
#  Debian world: sudo apt install build-essential
#  Arch world: sudo pacman -S base-devel

# Install Wild:
cargo install --locked wild-linker
```

### Build Puccini CLI Tools

```
git clone https://github.com/tliron/puccini.git
cd puccini

# You may want to checkout a specific release tag; for example:
git checkout v0.0.4

# Build!
scripts/build

# Verify it
puccini-tosca version --build

# Test it
puccini-tosca compile \
  examples/tour/data-types.yaml \
  --instantiate \
  --update
```

The default for `scripts/build` is a debug build:

* builds faster
* slow runtime performance: *no* optimizations enabled
* includes debug information (for error backtraces and interaction with debuggers)
* thus produces larger binaries

| Note that for debug builds only are using the compiler from the nightly channel of Rust. The only reason is that it's faster! It supports parallel builds and alternative (faster) linkers. Eventually these features will make it into stable Rust and we'll stop using nightly.

Unless you're developing Puccini, you'll likely prefer to use a release build:

* very slow to build, beware!
* best runtime performance: *all* optimizations enabled
* small binaries

```
scripts/build -r

# Verify it
puccini-tosca version --build
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
