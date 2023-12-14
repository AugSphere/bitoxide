mod bindgen;
mod compile_wasm;

use bindgen::generate_js_bindings;
use clap::{Parser, Subcommand, ValueEnum};
use std::{
    env,
    ffi::OsString,
    path::{Path, PathBuf},
    process::{ExitCode, ExitStatus},
};

/// xtask handler for generating WASM and js from workspace packages
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generates wasm js files for workspace packages
    Codegen {
        #[arg(long, default_value = "release")]
        profile: Profile,
    },
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Profile {
    /// Release mode, artifacts go in target/wasm32-unknown-unknown/release/
    Release,
    /// Dev mode, artifacts go in target/wasm32-unknown-unknown/debug/
    Dev,
}

impl Profile {
    fn artifact_stem(&self) -> String {
        match self {
            Profile::Release => "release".to_owned(),
            Profile::Dev => "debug".to_owned(),
        }
    }
}

impl ToString for Profile {
    fn to_string(&self) -> String {
        match self {
            Profile::Release => "release".to_owned(),
            Profile::Dev => "dev".to_owned(),
        }
    }
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let status = match cli.command {
        Commands::Codegen { profile } => codegen(profile),
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
