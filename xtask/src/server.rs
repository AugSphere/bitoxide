use log;
use std::fs;
use std::path::PathBuf;
use std::{fs::create_dir_all, net::SocketAddr, path::Path, process::ExitCode};

use async_std::net::TcpStream;
use async_tungstenite::{tungstenite::Message, WebSocketStream};
use futures::{
    channel::mpsc::channel, channel::mpsc::Receiver, executor::LocalPool, select,
    stream::StreamExt, task::LocalSpawnExt, FutureExt, SinkExt,
};

mod file_watcher;
use file_watcher::debouncing_file_watcher;
use file_watcher::{js_paths_in, WatchReceiver};

mod send_files;
use send_files::send_files;

mod rpc_types;
use self::rpc_types::{RpcRequest, RpcResponse};

pub fn launch_server(port: u16, watch_path: &Path) -> ExitCode {
    let mut quit_rx = set_ctrl_handler();

    if !watch_path.exists() {
        log::info!("Directory {watch_path:?} does not exist, creating it");
        create_dir_all(&watch_path).expect("Failed to create wasm_output dir");
    }

    log::info!("Setting up file watch on {watch_path:?}...");
    let (_watcher, watch_event_rx) = debouncing_file_watcher(&watch_path);

    let address = ([127, 0, 0, 1], port).into();
    let serve = async move {
        let Some(mut websocket) = connect(address, &mut quit_rx).await else {
            return;
        };
        stream_watched(&mut websocket, watch_event_rx, &mut quit_rx).await;
    };

    let mut pool = LocalPool::new();
    pool.spawner()
        .spawn_local(serve)
        .expect("Failed to set up file watcher task");
    pool.run();
    ExitCode::SUCCESS
}

pub fn get_definitions(port: u16, path: PathBuf) -> ExitCode {
    let mut quit_rx = set_ctrl_handler();

    let address = ([127, 0, 0, 1], port).into();
    let send_request = async move {
        let Some(mut websocket) = connect(address, &mut quit_rx).await else {
            return;
        };
        let definitions = request_definitions(&mut websocket).await;
        fs::write(&path, definitions).expect("Failed writing definitions to file");
        log::info!("Definitions written to {path:?}");
    };

    let mut pool = LocalPool::new();
    pool.spawner()
        .spawn_local(send_request)
        .expect("Failed to set up definition request task");
    pool.run();
    ExitCode::SUCCESS
}

fn set_ctrl_handler() -> futures::channel::mpsc::Receiver<()> {
    log::info!("Running, use Ctrl-C when you want to quit");
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

async fn connect(
    address: SocketAddr,
    quit_rx: &mut Receiver<()>,
) -> Option<WebSocketStream<TcpStream>> {
    log::info!("Listening on port {}...", address.port());
    let server = async_std::net::TcpListener::bind(address)
        .await
        .expect("Could not bind to port");

    let stream = select! {
        res = server.accept().fuse() => res.expect("Failed to establish a connection to Bitburner").0,
        _ = quit_rx.next() => return None,
    };

    let websocket = async_tungstenite::accept_async(stream)
        .await
        .expect("Failed to create a websocket");
    Some(websocket)
}

async fn request_definitions(websocket: &mut WebSocketStream<TcpStream>) -> String {
    let request = RpcRequest::get_definition_file(1);
    let message = serde_json::to_string(&request).expect("Failed to prepare getDefinitionFile request");
    websocket.send(Message::Text(message)).await.unwrap();
    let message = websocket.next().await.unwrap().unwrap();
    let text = match message {
        Message::Text(text) => Ok(text),
        _ => Err("Unexpected response type from Bitburner"),
    }
    .unwrap();
    let def_json =
        serde_json::from_str::<RpcResponse<String>>(&text).expect("Unexpected response contents");
    def_json.result
}

async fn stream_watched(
    websocket: &mut WebSocketStream<TcpStream>,
    mut watch_event_rx: WatchReceiver,
    quit_rx: &mut Receiver<()>,
) {
    log::info!(
        "Connected, will upload new js script files, run `cargo xtask codegen` to generate them"
    );
    loop {
        select! {
            events = watch_event_rx.next() => {
                let js_paths = js_paths_in(events.expect("Missing events in stream"));
                let result = send_files(websocket, js_paths).await;
                if let Err(err) = result {
                    log::error!("Failed to send files: {err}");
                }
            },
            _ = quit_rx.next() => break,
        }
    }
}
