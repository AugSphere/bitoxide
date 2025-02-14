use clap::Parser;
use std::process::ExitCode;
use xtask::{bindgen, cli, compile_wasm, get_wasm_artifact_paths, js_output_path, server};

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
    let code = match status.code() {
        Some(code) => match u8::try_from(code) {
            Ok(code) => ExitCode::from(code),
            _ => return ExitCode::FAILURE,
        },
        _ => return ExitCode::FAILURE,
    };
    let wasm_paths = get_wasm_artifact_paths(profile);
    bindgen::generate_js_bindings(profile, wasm_paths, &js_output_path());
    code
}
