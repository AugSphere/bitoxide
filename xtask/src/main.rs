use cargo::{
    core::compiler::{CompileKind, CompileMode, CompileTarget},
    core::package::Package,
    core::Workspace,
    ops::Packages,
    util::config::Config,
};
use clap::{Parser, Subcommand};
use std::{
    env,
    path::{Path, PathBuf},
    process::Command,
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
    compile_wasm();

    Ok(())
}

fn compile_wasm() -> Vec<Package> {
    let ignored_packages = vec!["bitoxide".to_owned(), "xtask".to_owned()];
    let cargo_config = Config::default().expect("Failed to create default cargo config");
    let workspace = Workspace::new(project_root().join("Cargo.toml").as_path(), &cargo_config)
        .expect("Failed to create a cargo workspace");
    let wasm_packages: Vec<Package> = workspace
        .members()
        .filter(|p| !ignored_packages.contains(&p.name().to_string()))
        .map(|p| p.to_owned())
        .collect();

    let mut compile_opts = cargo::ops::CompileOptions::new(&cargo_config, CompileMode::Build)
        .expect("Failed to create compile options");
    let wasm_target = CompileTarget::new("wasm32-unknown-unknown").unwrap();
    compile_opts.spec = Packages::OptOut(ignored_packages);
    compile_opts.build_config.requested_profile = "release".into();
    compile_opts.build_config.requested_kinds = vec![CompileKind::Target(wasm_target)];
    let _ = cargo::ops::compile(&workspace, &compile_opts).expect("WASM compilation failed");
    wasm_packages
}

fn project_root() -> PathBuf {
    Path::new(&env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(1)
        .unwrap()
        .to_path_buf()
}
