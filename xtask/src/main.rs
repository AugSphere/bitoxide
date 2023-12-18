use clap::Parser;
use std::process::ExitCode;
use xtask::{
    bindgen, cli, compile_wasm, exit_code_from_status, get_wasm_artifact_paths, js_output_path,
    server,
};

fn main() -> ExitCode {
    env_logger::builder()
        .target(env_logger::Target::Stdout)
        .filter_level(log::LevelFilter::Info)
        .init();
    let cli = cli::Cli::parse();
    match cli.command {
        cli::Commands::Codegen { profile } => codegen(profile),
        cli::Commands::Serve { port } => server::launch_server(port, &js_output_path()),
        cli::Commands::GetDefinitions { port, output } => server::get_definitions(port, output),
    }
}

fn codegen(profile: cli::Profile) -> ExitCode {
    let status = compile_wasm::compile_wasm_packages(profile);
    if !status.success() {
        return exit_code_from_status(status);
    }
    let wasm_paths = get_wasm_artifact_paths(profile);
    match bindgen::generate_js_bindings(profile, wasm_paths, &js_output_path()) {
        Ok(_) => ExitCode::SUCCESS,
        Err(status) => exit_code_from_status(status),
    }
}
