use std::path::PathBuf;

use cargo::{
    core::compiler::{CompileKind, CompileMode, CompileTarget, MessageFormat},
    core::Workspace,
    ops::Packages,
    util::config::Config,
};

pub fn compile_wasm_packages(project_root: PathBuf) -> Vec<String> {
    let ignored_packages = vec!["bitoxide".to_owned(), "xtask".to_owned()];
    let cargo_config = Config::default().expect("Failed to create default cargo config");
    let workspace = Workspace::new(project_root.join("Cargo.toml").as_path(), &cargo_config)
        .expect("Failed to create a cargo workspace");

    let mut compile_opts = cargo::ops::CompileOptions::new(&cargo_config, CompileMode::Build)
        .expect("Failed to create compile options");
    let wasm_target = CompileTarget::new("wasm32-unknown-unknown").unwrap();
    compile_opts.spec = Packages::OptOut(ignored_packages);
    compile_opts.build_config.requested_profile = "release".into();
    compile_opts.build_config.requested_kinds = vec![CompileKind::Target(wasm_target)];
    compile_opts.build_config.message_format = MessageFormat::Short;
    let compilation_results =
        cargo::ops::compile(&workspace, &compile_opts).expect("WASM compilation failed");
    compilation_results.root_crate_names
}
