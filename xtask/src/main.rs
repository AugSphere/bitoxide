mod bindgen;
mod cli;
mod compile_wasm;
mod server;

use clap::Parser;
use cli::Profile;

use bindgen::generate_js_bindings;
use server::launch_server;
use std::{
    env,
    ffi::OsString,
    path::{Path, PathBuf},
    process::ExitCode,
};

fn main() -> ExitCode {
    let cli = cli::Cli::parse();
    match cli.command {
        cli::Commands::Codegen { profile } => codegen(profile),
        cli::Commands::Serve { port } => launch_server(port, &js_output_path()),
    }
}

fn codegen(profile: Profile) -> ExitCode {
    let status = compile_wasm::compile_wasm_packages(profile);
    let wasm_paths = get_wasm_artifact_paths(profile);
    generate_js_bindings(profile, wasm_paths, &js_output_path());
    let code = status.code();
    match code {
        Some(code) => match u8::try_from(code) {
            Ok(code) => ExitCode::from(code),
            _ => ExitCode::FAILURE,
        },
        _ => ExitCode::FAILURE,
    }
}

pub fn project_root() -> PathBuf {
    Path::new(&env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(1)
        .unwrap()
        .to_path_buf()
}

pub fn artifact_path(profile: Profile) -> PathBuf {
    project_root()
        .join("target")
        .join("wasm32-unknown-unknown")
        .join(profile.artifact_stem())
}

pub fn js_output_path() -> PathBuf {
    project_root().join("target").join("wasm_output")
}

pub fn get_wasm_artifact_paths(profile: Profile) -> Vec<PathBuf> {
    let artifact_path = artifact_path(profile);
    let artifacts = artifact_path
        .read_dir()
        .expect("Could not read artifact directory");
    let wasm: OsString = "wasm".into();
    let mut wasm_paths = vec![];
    for file in artifacts {
        let filepath = file.expect("IO error iterating over artifacts").path();
        match (filepath.file_stem(), filepath.extension()) {
            (Some(_), Some(ext)) if ext == wasm => {
                wasm_paths.push(filepath);
            }
            _ => (),
        }
    }
    wasm_paths
}
