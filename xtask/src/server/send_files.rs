use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

use async_std::net::TcpStream;
use async_tungstenite::tungstenite::{self, Message};
use async_tungstenite::WebSocketStream;
use futures::stream::StreamExt;
use futures::SinkExt;
use log;

use super::{RpcRequest, RpcResponse};

type DynError = Box<dyn std::error::Error>;

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
    let reply = post_to_websocket(websocket, filename, contents).await?;
    let Message::Text(json) = reply else {
        return Err("Unexpected response type from Bitburner".into());
    };
    let response =
        serde_json::from_str::<RpcResponse<String>>(&json).expect("Unexpected response contents");
    log::info!("Sending {filename}: {}", response.result);
    Ok(())
}

async fn post_to_websocket(
    websocket: &mut WebSocketStream<TcpStream>,
    filename: &str,
    contents: String,
) -> Result<Message, tungstenite::Error> {
    let request = RpcRequest::push_file(1, "home", filename, contents);
    let message = serde_json::to_string(&request).expect("Failed to prepare pushFile request");
    websocket.send(Message::Text(message)).await?;
    let response = websocket.next().await.unwrap();
    response
}
