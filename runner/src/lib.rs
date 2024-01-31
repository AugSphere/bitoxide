use std::rc::Rc;
use std::time::Duration;

use bitburner_api::{wasm_bindgen, wasm_bindgen_futures, NS};
use gloo_timers::future::sleep;
use script_lib::*;

#[wasm_bindgen]
pub async fn main_rs(ns: NS) {
    let ns = Rc::new(ns);
    let now = || {
        let window = web_sys::window().expect("should have a window in this context");
        let performance = window
            .performance()
            .expect("performance should be available");
        performance.now()
    };

    let sleep_millis = |millis: f64| async move {
        sleep(Duration::from_millis(millis as u64)).await;
    };

    let host = ns.get_hostname();
    let max_ram = ns.get_server_max_ram(&host);
    let used_ram = ns.get_server_used_ram(&host);
    let free_ram = max_ram - used_ram;

    let mut executor = BitburnerExecutor::new(free_ram, now, sleep_millis);
    let executor_data =
        ExecutorData::new(now, executor.get_ram_cell(), executor.get_schedule_queue());

    for target in find_hackable_servers(&ns) {
        if ns.get_server_min_security_level(&target) < ns.get_server_security_level(&target) {
            let threads = get_threads_for_full_weaken(&ns, &target, 4);
            let weaken = weaken_process(
                ns.clone(),
                &target,
                Some(threads.into()),
                executor_data.clone(),
            );
            executor.register(Box::pin(weaken));
        }
    }

    let _ = ns.tail(None, None, vec![]);
    match executor.run().await {
        Ok(()) => {}
        Err(msg) => ns.print(&("ERROR ".to_owned() + &msg)),
    }
}
