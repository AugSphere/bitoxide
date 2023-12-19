pub mod bindgen;
pub mod cli;
pub mod compile_wasm;
pub mod server;

use std::env;
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::process::{ExitCode, ExitStatus};

use cli::Profile;

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

pub fn exit_code_from_status(status: ExitStatus) -> ExitCode {
    match status.code() {
        Some(code) => match u8::try_from(code) {
            Ok(code) => ExitCode::from(code),
            _ => return ExitCode::FAILURE,
        },
        _ => return ExitCode::FAILURE,
    }
}
