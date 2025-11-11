#![allow(unused)]

use std::{
    env,
    fs::*,
    io::{self, Read},
    path::*,
};

const WASM: &str = "puccini_plugin_tosca_2_0";

fn main() {
    #[cfg(not(feature = "_blanket"))]
    if let Err(error) = build(WASM) {
        println!("cargo::error={}", error);
    }
}

fn build(name: &str) -> Result<(), String> {
    println!("cargo::rerun-if-env-changed=WASM_PROFILE");

    let source_file = source_file(name)?;
    println!("cargo::warning=Bundling Floria plugin: {}", source_file.display());
    println!("cargo::rerun-if-changed={}", source_file.display());

    let target_file = target_file(name)?;

    #[cfg(feature = "wasm-precompiled")]
    precompile_wasm_file(&source_file, &target_file)?;

    #[cfg(not(feature = "wasm-precompiled"))]
    use_file(&source_file, &target_file)?;

    Ok(())
}

//
// Files
//

fn read_file(path: &Path) -> Result<Vec<u8>, String> {
    let mut file = io::BufReader::new(File::open(path).map_err(|_| format!("open: {}", path.display()))?);
    let mut bytes = Vec::default();
    file.read_to_end(&mut bytes).map_err(|_| format!("read: {}", path.display()));
    Ok(bytes)
}

#[cfg(unix)]
fn use_file(source_path: &Path, target_path: &Path) -> Result<(), String> {
    use std::os::unix::fs::*;

    println!("cargo::warning=Linking plugin to: {}", target_path.display());

    if let Err(error) = remove_file(&target_path)
        && (error.kind() != io::ErrorKind::NotFound)
    {
        return Err(error.to_string());
    }

    symlink(&source_path, &target_path)
        .map_err(|_| format!("cannot link {:?} to {:?}", source_path.display(), target_path.display()))
}

#[cfg(not(unix))]
fn use_file(source_path: &Path, target_path: &Path) -> Result<(), String> {
    println!("cargo::warning=Copying plugin to: {}", target_path.display());
    copy(&source_path, &target_path)
        .map_err(|_| format!("cannot copy {:?} to {:?}", source_path.display(), target_path.display()))?;
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
    let profile = match env::var("WASM_PROFILE") {
        Ok(profile) => profile,
        Err(_) => environment_string("PROFILE")?,
    };
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
    println!("cargo::warning=Precompiling plugin to: {}", target_path.display());
    let wasm = read_file(&source_path)?;
    let cwasm = precompile_wasm(&wasm)?;
    write(&target_path, &cwasm).map_err(|_| format!("write: {}", target_path.display()))
}

#[cfg(feature = "wasm-precompiled")]
fn precompile_wasm(wasm: &[u8]) -> Result<Vec<u8>, String> {
    #[cfg(feature = "wasm-debug")]
    let debug = true;
    #[cfg(not(feature = "wasm-debug"))]
    let debug = false;

    let environment =
        floria::plugins::PluginEnvironment::new(debug).map_err(|error| format!("wasmtime engine: {}", error))?;
    environment.engine.precompile_component(wasm).map_err(|error| format!("wasmtime precompile: {}", error))
}
