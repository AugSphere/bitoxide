use bitoxide::{file_watcher::debouncing_file_watcher, server_lib::serve};

use std::{fs::create_dir_all, path::Path};

use futures::{channel::mpsc::channel, executor::LocalPool, task::LocalSpawnExt};

pub fn main() {
    let (mut quit_tx, quit_rx) = channel::<()>(1);
    ctrlc::set_handler(move || {
        println!("");
        quit_tx
            .try_send(())
            .expect("Failed to send shutdown command")
    })
    .expect("Error setting Ctrl-C handler");
    println!("Starting server, use Ctrl-C to quit");

    let port_number: u16 = 7953;
    let path = Path::new(".").join("target").join("wasm_output");
    let path_str = path
        .to_str()
        .expect("Watched path can't be converted to string");

    if !path.exists() {
        println!("Directory {path_str} does not exist, creating it");
        create_dir_all(&path).expect("Failed to create wasm_output dir");
    }

    println!("Setting up file watch on {path_str}...");
    let (_debouncer, rx) = debouncing_file_watcher(&path);

    println!("Listening on port {port_number}...");
    let addr = format!("127.0.0.1:{}", port_number);
    let mut pool = LocalPool::new();
    pool.spawner()
        .spawn_local(serve(rx, quit_rx, addr))
        .expect("Failed to set up file watcher task");
    pool.run();
}
