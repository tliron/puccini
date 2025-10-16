#![allow(unused)]

use std::{env, fs::*, io::Read, path::*};

fn main() {
    let name = "puccini_plugin_tosca_2_0_functions";

    let source_path = source_path(name);
    println!("cargo::rerun-if-changed={}", source_path.display());
    let target_path = target_path(name);

    #[cfg(feature = "wasm-precompiled")]
    precompile_wasm_file(&source_path, &target_path);

    #[cfg(not(feature = "wasm-precompiled"))]
    copy_file(&source_path, &target_path);
}

fn dump_env() {
    let v: Vec<_> = env::vars().collect();
    panic!("{:#?}", v);
}

// Files

fn read_file(path: &Path) -> Vec<u8> {
    let mut file = File::open(path).expect(&format!("open: {}", path.display()));
    let mut bytes = Vec::default();
    file.read_to_end(&mut bytes).expect(&format!("read: {}", path.display()));
    bytes
}

fn copy_file(source_path: &Path, target_path: &Path) {
    copy(&source_path, &target_path).expect(&format!("copy: {} -> {}", source_path.display(), target_path.display()));
}

// Paths

#[cfg(feature = "bindeps")]
fn source_path(name: &str) -> PathBuf {
    // https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#artifact-dependencies
    let uppercase = name.to_uppercase();
    let name = format!("CARGO_CDYLIB_FILE_{}_{}", uppercase, name);
    let out_path: PathBuf = env::var(&name).expect(&name).into();
    out_path
}

#[cfg(not(feature = "bindeps"))]
fn source_path(name: &str) -> PathBuf {
    // We assume it was already built
    wasm_target_path().join(String::from(name) + ".wasm")
}

fn target_path(name: &str) -> PathBuf {
    #[cfg(feature = "wasm-precompiled")]
    let suffix = ".cwasm";

    #[cfg(not(feature = "wasm-precompiled"))]
    let suffix = ".wasm";

    out_path().join(String::from(name) + suffix)
}

fn wasm_target_path() -> PathBuf {
    let cargo_manifest_path: PathBuf = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR").into();
    let profile = env::var("PROFILE").expect("PROFILE");
    cargo_manifest_path.join("..").join("target").join("wasm32-wasip2").join(profile)
}

fn out_path() -> PathBuf {
    let out_path: PathBuf = env::var("OUT_DIR").expect("OUT_DIR").into();
    out_path
}

// Precompile

#[cfg(feature = "wasm-precompiled")]
fn precompile_wasm_file(source_path: &Path, target_path: &Path) {
    let wasm = read_file(&source_path);
    let cwasm = precompile_wasm(&wasm);
    write(&target_path, &cwasm).expect(&format!("write: {}", target_path.display()));
}

#[cfg(feature = "wasm-precompiled")]
fn precompile_wasm(wasm: &[u8]) -> Vec<u8> {
    use wasmtime::*;

    #[allow(unused_mut)]
    let mut config = Config::new();

    #[cfg(feature = "wasm-debug-info")]
    config.debug_info(true);

    let engine = Engine::new(&config).expect("wasmtime engine");
    let precompiled = engine.precompile_component(wasm).expect("wasmtime precompile");
    precompiled
}
