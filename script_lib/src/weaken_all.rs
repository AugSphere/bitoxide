use std::rc::Rc;

use bitburner_api::NS;

use crate::find_servers::find_hackable_servers;
use crate::thread_search::get_threads_for_full_weaken;
use crate::{weaken_process, BitburnerExecutor, ExecutorData, SleepFuture};

pub async fn weaken_all<F>(
    ns: Rc<NS>,
    cores: u32,
    now: fn() -> f64,
    sleep_millis: fn(f64) -> F,
) -> Result<(), String>
where
    F: SleepFuture,
{
    ns.disable_log("ALL");

    let host = ns.get_hostname();
    let max_ram = ns.get_server_max_ram(&host);
    let used_ram = ns.get_server_used_ram(&host);
    let free_ram = max_ram - used_ram;

    let mut executor = BitburnerExecutor::new(free_ram, now, sleep_millis);
    let executor_data =
        ExecutorData::new(now, executor.get_ram_cell(), executor.get_schedule_queue());

    let hackable_servers = find_hackable_servers(&ns);
    for target in hackable_servers {
        if ns.get_server_min_security_level(&target) < ns.get_server_security_level(&target) {
            let threads = get_threads_for_full_weaken(&ns, &target, cores);
            let weaken = weaken_process(
                ns.clone(),
                &target,
                Some(threads.into()),
                executor_data.clone(),
            );
            executor.register(Box::pin(weaken));
        }
    }

    ns.enable_log("ALL");
    executor.run().await
}
