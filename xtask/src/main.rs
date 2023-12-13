use clap::{Parser, Subcommand};
use std::{
    env,
    path::{Path, PathBuf},
};
mod bindgen;
mod compile_wasm;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generates wasm js files for child crates
    Codegen {
        #[arg(long, default_value = "release")]
        profile: String,
    },
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Codegen { profile } => codegen(&profile),
    }
}

fn codegen(profile: &str) {
    let root = project_root();
    let wasm_package_names = compile_wasm::compile_wasm_packages(&root);
    let crate_target_dir = root.join("target");
    for package_name in wasm_package_names {
        bindgen::wasm_to_js(&package_name, &crate_target_dir, &profile);
    }
}

fn project_root() -> PathBuf {
    Path::new(&env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(1)
        .unwrap()
        .to_path_buf()
}
