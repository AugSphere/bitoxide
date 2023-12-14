mod bindgen;
mod cli;
mod compile_wasm;

use clap::Parser;
use cli::Profile;

use bindgen::generate_js_bindings;
use std::{
    env,
    ffi::OsString,
    path::{Path, PathBuf},
    process::{ExitCode, ExitStatus},
};

fn main() -> ExitCode {
    let cli = cli::Cli::parse();
    let status = match cli.command {
        cli::Commands::Codegen { profile } => codegen(profile),
    };
    let code = status.code();
    match code {
        Some(code) => match u8::try_from(code) {
            Ok(code) => ExitCode::from(code),
            _ => ExitCode::FAILURE,
        },
        _ => ExitCode::FAILURE,
    }
}

fn codegen(profile: Profile) -> ExitStatus {
    let status = compile_wasm::compile_wasm_packages(profile);
    let wasm_paths = get_wasm_artifact_paths(profile);
    generate_js_bindings(profile, wasm_paths);
    status
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
