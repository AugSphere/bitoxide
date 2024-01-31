use std::rc::Rc;

use bitburner_api::netscript::ThreadOrOptions;
use bitburner_api::NS;

mod executor;
pub use executor::{BitburnerExecutor, SleepFuture};

mod reactor;
mod waker;

mod process;
use process::BitburnerProcess;

pub use self::process::ExecutorData;

pub fn hack_process(
    ns: Rc<NS>,
    host: &str,
    thread_or_options: Option<ThreadOrOptions>,
    executor_data: ExecutorData,
) -> BitburnerProcess {
    process(ns, Script::Hack, host, thread_or_options, executor_data)
}

pub fn grow_process(
    ns: Rc<NS>,
    host: &str,
    thread_or_options: Option<ThreadOrOptions>,
    executor_data: ExecutorData,
) -> BitburnerProcess {
    process(ns, Script::Grow, host, thread_or_options, executor_data)
}

pub fn weaken_process(
    ns: Rc<NS>,
    host: &str,
    thread_or_options: Option<ThreadOrOptions>,
    executor_data: ExecutorData,
) -> BitburnerProcess {
    process(ns, Script::Weaken, host, thread_or_options, executor_data)
}

enum Script {
    Hack,
    Grow,
    Weaken,
}

fn process(
    ns: Rc<NS>,
    script: Script,
    host: &str,
    thread_or_options: Option<ThreadOrOptions>,
    executor_data: ExecutorData,
) -> BitburnerProcess {
    let (script, duration_hint, ram_hint_per_thread) = match script {
        Script::Hack => ("hack.js", ns.get_hack_time(host), 1.70),
        Script::Grow => ("grow.js", ns.get_grow_time(host), 1.75),
        Script::Weaken => ("weaken.js", ns.get_weaken_time(host), 1.75),
    };
    let threads = ThreadOrOptions::threads(&thread_or_options);
    BitburnerProcess::new(
        ns,
        script.to_owned(),
        thread_or_options,
        vec![host.into()],
        duration_hint,
        ram_hint_per_thread * threads as f64,
        executor_data,
    )
}
