use std::{
    fs::File,
    io::Read,
    path::{Path, PathBuf}, net::SocketAddr,
};

use async_std::net::TcpStream;

use futures::{channel::mpsc::Receiver, select, stream::StreamExt, FutureExt, SinkExt};

use async_tungstenite::{
    tungstenite::{self, Message},
    WebSocketStream,
};

use crate::file_watcher::{js_paths_in, WatchReceiver};

type DynError = Box<dyn std::error::Error>;

pub async fn serve(mut watch_event_rx: WatchReceiver, mut quit_rx: Receiver<()>, address: SocketAddr) {
    let server = async_std::net::TcpListener::bind(address)
        .await
        .expect("Could not bind to port");

    let stream = select! {
        res = server.accept().fuse() => res.expect("Failed to establish a connection to Bitburner").0,
        _ = quit_rx.next() => return,
    };

    let mut websocket = async_tungstenite::accept_async(stream)
        .await
        .expect("Failed to create a websocket");

    println!("Connected, will upload new js script files, run `cargo xtask codegen` to generate them");
    loop {
        select! {
            events = watch_event_rx.next() => {
                let js_paths = js_paths_in(events.expect("Missing events in stream"));
                let result = send_files(&mut websocket, js_paths).await;
                if let Err(err) = result {
                    eprintln!("Failed to send files: {err}");
                }
            },
            _ = quit_rx.next() => break,
        }
    }
}

pub async fn send_files(
    websocket: &mut WebSocketStream<TcpStream>,
    paths: Vec<PathBuf>,
) -> Result<(), DynError> {
    let mut results = vec![];
    for path in paths {
        let result = send_single_file(websocket, &path).await;
        results.push(result);
    }
    results.into_iter().collect()
}

async fn send_single_file(
    websocket: &mut WebSocketStream<TcpStream>,
    path: &Path,
) -> Result<(), DynError> {
    let mut file = File::open(&path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    drop(file);
    let filename = path
        .file_name()
        .ok_or("Not a file")?
        .to_str()
        .expect("Invalid filename");
    let reply = post_to_websocket(websocket, filename, &contents).await?;
    println!("{filename}: {reply}");
    Ok(())
}

async fn post_to_websocket(
    websocket: &mut WebSocketStream<TcpStream>,
    filename: &str,
    contents: &str,
) -> Result<Message, tungstenite::Error> {
    let message = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "pushFile",
        "params": {
            "filename": filename,
            "content": contents,
            "server": "home",
        }
    })
    .to_string();

    websocket.send(Message::Text(message)).await?;
    let response = websocket.next().await.unwrap();
    response
}
