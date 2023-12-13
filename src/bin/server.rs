use std::{
    ffi::OsString,
    fs::File,
    io::Read,
    net::TcpStream,
    path::{Path, PathBuf},
    time::Duration,
};

use futures::{
    channel::mpsc::{channel, Receiver},
    executor::LocalPool,
    select,
    task::LocalSpawnExt,
};
use notify_debouncer_mini::notify::{self, RecursiveMode};
use notify_debouncer_mini::{new_debouncer_opt, Config};

use futures::stream::StreamExt;
use tungstenite::{Message, WebSocket};

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

fn send_file(websocket: &mut WebSocket<TcpStream>, path: &Path) -> Result<(), DynError> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    drop(file);
    let filename = path
        .file_name()
        .ok_or("Not a file")?
        .to_str()
        .expect("Invalid filename");
    let reply = post_to_websocket(websocket, filename, &contents)?;
    println!("{:?}: {reply}", path.file_name());
    Ok(())
}

fn post_to_websocket(
    websocket: &mut WebSocket<TcpStream>,
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

    websocket.send(Message::Text(message))?;
    websocket.read()
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

    let port_number: u16 = 7953;
    let path = Path::new(".").join("target").join("wasm_output");
    let path_str = path
        .to_str()
        .expect("watch path can't be converted to string");

    let server = std::net::TcpListener::bind(&*format!("127.0.0.1:{}", port_number)).unwrap();

    println!("Listening on port {port_number}...");

    let stream = server.incoming().next().unwrap();
    let mut websocket = tungstenite::accept(stream.unwrap()).unwrap();
    let mut send_to_socket =
        move |path: PathBuf| -> Result<(), DynError> { send_file(&mut websocket, &path) };

    println!("Listener found. Watching and uploading files from {path_str}...",);

    let (_debouncer, mut rx) = debouncing_file_watcher(&path);

    let mut pool = LocalPool::new();
    let watch = async move {
        loop {
            select! {
                events = rx.next() => {
                    let js_paths = js_paths_in(events.expect("Missing events in stream"));
                    let send_all = js_paths.into_iter().map(&mut send_to_socket);
                    let result: Result<Vec<_>, DynError> = send_all.into_iter().collect();
                    if let Err(err) = result {
                        eprintln!("Failed to send files: {err}");
                    }
                },
                _ = quit_rx.next() => break,
            }
        }
    };
    pool.spawner()
        .spawn_local(watch)
        .expect("Failed to set up file watcher task");
    pool.run();
}
