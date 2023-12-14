use clap::Parser;
use std::process::ExitCode;
use xtask::{bindgen, cli, compile_wasm, get_wasm_artifact_paths, js_output_path, server};

fn main() -> ExitCode {
    let cli = cli::Cli::parse();
    match cli.command {
        cli::Commands::Codegen { profile } => codegen(profile),
        cli::Commands::Serve { port } => server::launch_server(port, &js_output_path()),
    }
}

fn codegen(profile: cli::Profile) -> ExitCode {
    let status = compile_wasm::compile_wasm_packages(profile);
    let wasm_paths = get_wasm_artifact_paths(profile);
    bindgen::generate_js_bindings(profile, wasm_paths, &js_output_path());
    let code = status.code();
    match code {
        Some(code) => match u8::try_from(code) {
            Ok(code) => ExitCode::from(code),
            _ => ExitCode::FAILURE,
        },
        _ => ExitCode::FAILURE,
    }
}
