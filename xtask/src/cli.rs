use clap::{Parser, Subcommand, ValueEnum};

/// xtask handler for generating WASM and js from workspace packages
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
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
    pub fn artifact_stem(&self) -> String {
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
