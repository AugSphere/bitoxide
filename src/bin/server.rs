use std::{path::Path, time::Duration};

use futures::{
    channel::mpsc::{channel, Receiver},
    executor::LocalPool,
    future::{self, select},
    pin_mut, select,
    task::SpawnExt,
};
use notify::{self, RecursiveMode};
use notify_debouncer_mini::{new_debouncer_opt, Config};

type DebouncedPollWatcher = notify_debouncer_mini::Debouncer<notify::PollWatcher>;
type WatcherEvents = Vec<notify_debouncer_mini::DebouncedEvent>;
type WatchReceiver = Receiver<Result<WatcherEvents, notify::Error>>;

use futures::stream::StreamExt;

fn debouncing_file_watcher() -> (DebouncedPollWatcher, WatchReceiver) {
    let (mut tx, rx) = channel(2);
    let backend_config = notify::Config::default().with_poll_interval(Duration::from_secs(1));
    let debouncer_config = Config::default()
        .with_timeout(Duration::from_millis(1000))
        .with_notify_config(backend_config);
    let watcher = new_debouncer_opt::<_, notify::PollWatcher>(debouncer_config, move |res| {
        futures::executor::block_on(async {
            tx.try_send(res)
                .expect("Failed to send path through the channel");
        })
    })
    .unwrap();
    (watcher, rx)
}

type DynError = Box<dyn std::error::Error>;

fn process_events(events: Result<WatcherEvents, notify::Error>) {
    println!("{:?}", events);
}

pub fn main() {
    let (mut quit_tx, mut quit_rx) = channel::<()>(1);
    ctrlc::set_handler(move || {
        println!("\nSetting quit flag");
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

    eprintln!(
        "Listener found. Watching and uploading files from {}...",
        path_str
    );

    let (mut debouncer, mut rx) = debouncing_file_watcher();
    debouncer
        .watcher()
        .watch(&path, RecursiveMode::NonRecursive)
        .unwrap();

    let mut pool = LocalPool::new();
    let watch = async move {
        loop {
            select! {
                events = rx.next() => process_events(events.expect("Missing events in stream")),
                _ = quit_rx.next() => break,
            }
        }
    };
    pool.spawner()
        .spawn(watch)
        .expect("Failed to set up file watcher task");
    pool.run();
    println!("done");
}
