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

    let mut executor = BitburnerExecutor::new(15.0, now, sleep_millis);
    let executor_data =
        ExecutorData::new(now, executor.get_ram_cell(), executor.get_schedule_queue());
    let hack = Box::pin(hack_process(
        ns.clone(),
        "home",
        None,
        executor_data.clone(),
    ));
    executor.register(hack);
    let weaken = Box::pin(weaken_process(ns.clone(), "home", None, executor_data));
    executor.register(weaken);
    let _ = ns.tail(None, None, vec![]);
    match executor.run().await {
        Ok(()) => {}
        Err(msg) => ns.print(&("ERROR ".to_owned() + &msg)),
    }
}
