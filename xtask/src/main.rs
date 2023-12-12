use clap::{Parser, Subcommand};
use std::{
    env,
    path::{Path, PathBuf},
};
mod bindgen;
mod compile_wasm;

type DynError = Box<dyn std::error::Error>;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generates wasm js files for child crates
    Codegen {},
    /// Launch server that passes js files to bitburner
    LaunchServer {},
}

fn main() -> Result<(), DynError> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Codegen {} => codegen()?,
        Commands::LaunchServer {} => (),
    }
    Ok(())
}

fn codegen() -> Result<(), DynError> {
    let wasm_package_names = compile_wasm::compile_wasm_packages(project_root());
    for package_name in wasm_package_names {
        let args = bindgen::PostBuildArgs {
            crate_name: package_name.to_owned(),
            crate_target_dir: project_root().join("target"),
            profile: "release".to_owned(),
            target_triple: "wasm32-unknown-unknown".to_owned(),
            debug: false,
        };
        bindgen::wasm_to_js(args);
    }

    Ok(())
}

fn project_root() -> PathBuf {
    Path::new(&env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(1)
        .unwrap()
        .to_path_buf()
}
