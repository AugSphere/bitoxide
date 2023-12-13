use std::{
    ffi::OsString,
    fs::{create_dir_all, File},
    io::Read,
    path::{Path, PathBuf},
    time::Duration,
};

use async_std::net::TcpStream;

use futures::{
    channel::mpsc::{channel, Receiver},
    executor::LocalPool,
    select,
    stream::StreamExt,
    task::LocalSpawnExt,
    FutureExt, SinkExt,
};
use notify_debouncer_mini::notify::{self, RecursiveMode};
use notify_debouncer_mini::{new_debouncer_opt, Config};

use async_tungstenite::{
    tungstenite::{self, Message},
    WebSocketStream,
};

type DebouncedPollWatcher = notify_debouncer_mini::Debouncer<notify::PollWatcher>;
type WatcherEvents = Vec<notify_debouncer_mini::DebouncedEvent>;
type WatchReceiver = Receiver<Result<WatcherEvents, notify::Error>>;

fn debouncing_file_watcher(path: &Path) -> (DebouncedPollWatcher, WatchReceiver) {
    let (mut tx, rx) = channel(2);
    let backend_config = notify::Config::default().with_poll_interval(Duration::from_secs(1));
    let debouncer_config = Config::default()
        .with_timeout(Duration::from_millis(1000))
        .with_notify_config(backend_config);
    let mut debouncer = new_debouncer_opt::<_, notify::PollWatcher>(debouncer_config, move |res| {
        futures::executor::block_on(async {
            tx.try_send(res)
                .expect("Failed to send path through the channel");
        })
    })
    .unwrap();

    debouncer
        .watcher()
        .watch(path, RecursiveMode::NonRecursive)
        .unwrap();
    (debouncer, rx)
}

type DynError = Box<dyn std::error::Error>;

fn js_paths_in(events: Result<WatcherEvents, notify::Error>) -> Vec<PathBuf> {
    let events = events.expect("File watcher error");
    events
        .into_iter()
        .filter_map(|event| {
            let path = event.path;
            let js_os: OsString = "js".into();
            let has_js_ext = path.extension() == Some(&js_os);
            (path.exists() & path.is_file() & has_js_ext).then_some(path)
        })
        .collect()
}

async fn send_files(
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

pub fn main() {
    let (mut quit_tx, mut quit_rx) = channel::<()>(1);
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
    let (_debouncer, mut rx) = debouncing_file_watcher(&path);

    println!("Listening on port {port_number}...");
    let addr = format!("127.0.0.1:{}", port_number);
    let watch = async move {
        let server = async_std::net::TcpListener::bind(addr)
            .await
            .expect("Could not bind to port");

        let stream = select! {
            res = server.accept().fuse() => res.expect("Failed to establish a connection to Bitburner").0,
            _ = quit_rx.next() => return,
        };

        let mut websocket = async_tungstenite::accept_async(stream)
            .await
            .expect("Failed to create a websocket");

        println!("Connected, uploading changed js scripts from wasm_output");
        loop {
            select! {
                events = rx.next() => {
                    let js_paths = js_paths_in(events.expect("Missing events in stream"));
                    let result = send_files(&mut websocket, js_paths).await;
                    if let Err(err) = result {
                        eprintln!("Failed to send files: {err}");
                    }
                },
                _ = quit_rx.next() => break,
            }
        }
    };
    let mut pool = LocalPool::new();
    pool.spawner()
        .spawn_local(watch)
        .expect("Failed to set up file watcher task");
    pool.run();
}
