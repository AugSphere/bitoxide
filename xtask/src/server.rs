use std::fs;
use std::fs::create_dir_all;
use std::net::{Ipv4Addr, SocketAddr};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use async_std::net::TcpStream;
use async_tungstenite::tungstenite::Message;
use async_tungstenite::WebSocketStream;
use futures::channel::mpsc::{channel, Receiver, Sender};
use futures::executor::LocalPool;
use futures::stream::StreamExt;
use futures::task::LocalSpawnExt;
use futures::SinkExt;
use log;

mod file_watcher;
use file_watcher::{debouncing_file_watcher, js_paths_in, WatchReceiver};

mod send_files;
use send_files::send_files;

mod rpc_types;
use self::rpc_types::{RpcRequest, RpcResponse};

pub fn launch_server(port: u16, watch_path: &Path) -> ExitCode {
    let (_, mut quit_rx) = set_ctrl_handler();

    if !watch_path.exists() {
        log::info!("Directory {watch_path:?} does not exist, creating it");
        create_dir_all(&watch_path).expect("Failed to create wasm_output dir");
    }

    log::info!("Setting up file watch on {watch_path:?}...");
    let (_watcher, watch_event_rx) = debouncing_file_watcher(&watch_path);

    let address = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), port);
    let serve = async move {
        let websocket = connect(address).await;
        stream_watched(websocket, watch_event_rx).await;
    };

    let mut pool = LocalPool::new();
    pool.spawner()
        .spawn_local(serve)
        .expect("Failed to set up file watcher task");
    pool.run_until(quit_rx.next());
    ExitCode::SUCCESS
}

pub fn get_definitions(port: u16, path: PathBuf) -> ExitCode {
    let (mut quit_tx, mut quit_rx) = set_ctrl_handler();

    let address = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), port);
    let send_request = async move {
        let websocket = connect(address).await;
        let definitions = request_definitions(websocket).await;
        fs::write(&path, definitions).expect("Failed writing definitions to file");
        log::info!("Definitions written to {path:?}");
        quit_tx
            .send(())
            .await
            .expect("Failed to send shutdown command")
    };

    let mut pool = LocalPool::new();
    pool.spawner()
        .spawn_local(send_request)
        .expect("Failed to set up definition request task");
    pool.run_until(quit_rx.next());
    ExitCode::SUCCESS
}

fn set_ctrl_handler() -> (Sender<()>, Receiver<()>) {
    log::info!("Running, use Ctrl-C when you want to quit");
    let (quit_tx, quit_rx) = channel::<()>(1);
    let mut ctrlc_tx = quit_tx.clone();
    ctrlc::set_handler(move || {
        println!("");
        ctrlc_tx
            .try_send(())
            .expect("Failed to send shutdown command")
    })
    .expect("Error setting Ctrl-C handler");
    (quit_tx, quit_rx)
}

async fn connect(address: SocketAddr) -> WebSocketStream<TcpStream> {
    log::info!("Listening on port {}...", address.port());
    let server = async_std::net::TcpListener::bind(address)
        .await
        .expect("Could not bind to port");
    let stream = server
        .accept()
        .await
        .expect("Failed to establish a connection to Bitburner")
        .0;
    let websocket = async_tungstenite::accept_async(stream)
        .await
        .expect("Failed to create a websocket");
    websocket
}

async fn request_definitions(mut websocket: WebSocketStream<TcpStream>) -> String {
    let request = RpcRequest::get_definition_file(1);
    let message =
        serde_json::to_string(&request).expect("Failed to prepare getDefinitionFile request");
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
    mut websocket: WebSocketStream<TcpStream>,
    mut watch_event_rx: WatchReceiver,
) {
    log::info!(
        "Connected, will upload new js script files, run `cargo xtask codegen` to generate them"
    );
    loop {
        let events = watch_event_rx.next().await;
        let js_paths = js_paths_in(events.expect("Missing events in stream"));
        let result = send_files(&mut websocket, js_paths).await;
        if let Err(err) = result {
            log::error!("Failed to send files: {err}");
        }
    }
}
