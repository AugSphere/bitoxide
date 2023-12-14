use clap::Parser;
use futures::{channel::mpsc::channel, executor::LocalPool, task::LocalSpawnExt};
use std::{fs::create_dir_all, path::Path};

use crate::{file_watcher::debouncing_file_watcher, server::serve};

mod file_watcher;
mod server;

/// A server to watch wasm output and upload it to Bitburner
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, default_value_t = 7953)]
    /// TCP port used for the Bitburner connection
    port: u16,
}

pub fn main() {
    let cli = Cli::parse();
    let port_number: u16 = cli.port;

    println!("Starting the server, use Ctrl-C to quit");
    let quit_rx = set_ctrl_handler();

    let wasm_path = Path::new(&env!("CARGO_MANIFEST_DIR"))
        .join("target")
        .join("wasm_output");

    if !wasm_path.exists() {
        println!("Directory {wasm_path:?} does not exist, creating it");
        create_dir_all(&wasm_path).expect("Failed to create wasm_output dir");
    }

    println!("Setting up file watch on {wasm_path:?}...");
    let (_watcher, rx) = debouncing_file_watcher(&wasm_path);

    println!("Listening on port {port_number}...");
    let address = ([127, 0, 0, 1], port_number).into();
    let mut pool = LocalPool::new();
    pool.spawner()
        .spawn_local(serve(rx, quit_rx, address))
        .expect("Failed to set up file watcher task");
    pool.run();
}

fn set_ctrl_handler() -> futures::channel::mpsc::Receiver<()> {
    let (mut quit_tx, quit_rx) = channel::<()>(1);
    ctrlc::set_handler(move || {
        println!("");
        quit_tx
            .try_send(())
            .expect("Failed to send shutdown command")
    })
    .expect("Error setting Ctrl-C handler");
    quit_rx
}
