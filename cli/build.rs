#![allow(unused)]

use std::{env, fs::*, io::Read, path::*};

fn main() {
    #[cfg(not(feature = "_blanket"))]
    if let Err(error) = build("puccini_plugin_tosca_2_0_functions") {
        println!("cargo::error={}", error);
    }
}

fn build(name: &str) -> Result<(), String> {
    let source_file = source_file(name)?;
    println!("cargo::rerun-if-changed={}", source_file.display());
    let target_file = target_file(name)?;

    #[cfg(feature = "wasm-precompiled")]
    precompile_wasm_file(&source_file, &target_file)?;

    #[cfg(not(feature = "wasm-precompiled"))]
    copy_file(&source_file, &target_file)?;

    Ok(())
}

//
// Files
//

fn read_file(path: &Path) -> Result<Vec<u8>, String> {
    let mut file = File::open(path).map_err(|_| format!("open: {}", path.display()))?;
    let mut bytes = Vec::default();
    file.read_to_end(&mut bytes).map_err(|_| format!("read: {}", path.display()));
    Ok(bytes)
}

fn copy_file(source_path: &Path, target_path: &Path) -> Result<(), String> {
    copy(&source_path, &target_path)
        .map_err(|_| format!("copy: {:?} -> {:?}", source_path.display(), target_path.display()))?;
    Ok(())
}

#[cfg(feature = "bindeps")]
fn source_file(name: &str) -> Result<PathBuf, String> {
    // It was built by Cargo as an artifact build-dependency
    // https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#artifact-dependencies
    environment_file(&format!("CARGO_CDYLIB_FILE_{}_{}", name.to_uppercase(), name))
}

#[cfg(not(feature = "bindeps"))]
fn source_file(name: &str) -> Result<PathBuf, String> {
    // We assume it's already built
    Ok(external_build_dir()?.join(String::from(name) + ".wasm"))
}

fn target_file(name: &str) -> Result<PathBuf, String> {
    #[cfg(feature = "wasm-precompiled")]
    let suffix = ".cwasm";

    #[cfg(not(feature = "wasm-precompiled"))]
    let suffix = ".wasm";

    Ok(current_build_dir()?.join(String::from(name) + suffix))
}

//
// Directories
//

fn current_build_dir() -> Result<PathBuf, String> {
    environment_file("OUT_DIR")
}

fn external_build_dir() -> Result<PathBuf, String> {
    let cargo_manifest_path = environment_file("CARGO_MANIFEST_DIR")?;
    let profile = environment_string("PROFILE")?;
    Ok(cargo_manifest_path.join("..").join("target").join("wasm32-wasip2").join(profile))
}

//
// Environment
//

fn environment_string(name: &str) -> Result<String, String> {
    Ok(env::var(name).map_err(|_| format!("variable not set: {}", name))?)
}

fn environment_file(name: &str) -> Result<PathBuf, String> {
    Ok(environment_string(name)?.into())
}

fn dump_environment() {
    let vars: Vec<_> = env::vars().collect();
    panic!("{:#?}", vars);
}

//
// Wasm
//

#[cfg(feature = "wasm-precompiled")]
fn precompile_wasm_file(source_path: &Path, target_path: &Path) -> Result<(), String> {
    let wasm = read_file(&source_path)?;
    let cwasm = precompile_wasm(&wasm)?;
    write(&target_path, &cwasm).map_err(|_| format!("write: {}", target_path.display()))
}

#[cfg(feature = "wasm-precompiled")]
fn precompile_wasm(wasm: &[u8]) -> Result<Vec<u8>, String> {
    use wasmtime::*;

    #[allow(unused_mut)]
    let mut config = Config::new();

    #[cfg(feature = "wasm-debug-info")]
    config.debug_info(true);

    let engine = Engine::new(&config).expect("wasmtime engine");
    let precompiled = engine.precompile_component(wasm).map_err(|error| format!("wasmtime precompile: {}", error));
    precompiled
}
