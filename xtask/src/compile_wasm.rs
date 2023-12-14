use std::process::{Command, ExitStatus};

use crate::Profile;

pub fn compile_wasm_packages(profile: Profile) -> ExitStatus {
    let ignored_packages = vec!["bitoxide", "xtask", "bitburner_api"];

    let mut command = Command::new("cargo");
    command
        .arg("build")
        .arg("--target")
        .arg("wasm32-unknown-unknown")
        .arg("--profile")
        .arg(profile.to_string())
        .arg("--workspace");
    for ignored in ignored_packages {
        command.arg("--exclude").arg(ignored);
    }
    command.status().expect("Cannot run cargo build")
}
