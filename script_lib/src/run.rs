use std::future::Future;

use bitburner_api::netscript::ThreadOrOptions;
use bitburner_api::NS;

mod executor;
use executor::ConstrainedPriorityExecutor;

mod reactor;
mod waker;

mod process;
use process::BitburnerProcess;

pub fn hack_process<'a, F: Future<Output = ()>>(
    ns: &'a NS,
    executor: &'a ConstrainedPriorityExecutor<F>,
    host: &str,
    thread_or_options: Option<ThreadOrOptions>,
) -> BitburnerProcess<'a, F> {
    process(ns, executor, Script::Hack, host, thread_or_options)
}

pub fn grow_process<'a, F: Future<Output = ()>>(
    ns: &'a NS,
    executor: &'a ConstrainedPriorityExecutor<F>,
    host: &str,
    thread_or_options: Option<ThreadOrOptions>,
) -> BitburnerProcess<'a, F> {
    process(ns, executor, Script::Grow, host, thread_or_options)
}

pub fn weaken_process<'a, F: Future<Output = ()>>(
    ns: &'a NS,
    executor: &'a ConstrainedPriorityExecutor<F>,
    host: &str,
    thread_or_options: Option<ThreadOrOptions>,
) -> BitburnerProcess<'a, F> {
    process(ns, executor, Script::Weaken, host, thread_or_options)
}

enum Script {
    Hack,
    Grow,
    Weaken,
}

fn process<'a, F: Future<Output = ()>>(
    ns: &'a NS,
    executor: &'a ConstrainedPriorityExecutor<F>,
    script: Script,
    host: &str,
    thread_or_options: Option<ThreadOrOptions>,
) -> BitburnerProcess<'a, F> {
    let (script, duration_hint, ram_hint_per_thread) = match script {
        Script::Hack => ("hack.js", ns.get_hack_time(host), 1.70),
        Script::Grow => ("grow.js", ns.get_grow_time(host), 1.75),
        Script::Weaken => ("weaken.js", ns.get_weaken_time(host), 1.75),
    };
    let threads = ThreadOrOptions::threads(&thread_or_options);
    BitburnerProcess::new(
        ns,
        executor,
        script.to_owned(),
        thread_or_options,
        vec![host.into()],
        duration_hint,
        ram_hint_per_thread * threads as f64,
    )
}
