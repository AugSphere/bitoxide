use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::time::Duration;

use futures::channel::mpsc::{channel, Receiver};
use notify_debouncer_mini::notify::{self, RecursiveMode};
use notify_debouncer_mini::{new_debouncer_opt, Config};

type DebouncedPollWatcher = notify_debouncer_mini::Debouncer<notify::PollWatcher>;
type WatcherEvents = Vec<notify_debouncer_mini::DebouncedEvent>;
pub type WatchReceiver = Receiver<Result<WatcherEvents, notify::Error>>;

pub fn debouncing_file_watcher(path: &Path) -> (DebouncedPollWatcher, WatchReceiver) {
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

pub fn js_paths_in(events: Result<WatcherEvents, notify::Error>) -> Vec<PathBuf> {
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
