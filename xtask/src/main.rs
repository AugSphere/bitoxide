use clap::{Parser, Subcommand};
use std::{
    env, fs,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

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
    println!("Doing codegen!");
    let cargo = env::var("CARGO").unwrap_or_else(|_| "cargo".to_string());
    let status = Command::new(cargo)
        .current_dir(project_root())
        .args(&["build", "--release"])
        .status()?;

    if !status.success() {
        Err("cargo build failed")?;
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
