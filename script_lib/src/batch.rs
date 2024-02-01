use bitburner_api::NS;

use crate::thread_search::weaken_analyze_threads;

#[derive(Debug, Clone, Copy)]
pub struct BatchThreads {
    pub hack: u32,
    pub hack_weaken: u32,
    pub grow: u32,
    pub grow_weaken: u32,
}

pub fn calculate_batch_threads(
    ns: &NS,
    hack_amount: f64,
    target: &str,
    cores: u32,
) -> BatchThreads {
    let cores = Some(cores);
    let money = ns.get_server_money_available(target);
    let hack_threads: u32 = (hack_amount / (money * ns.hack_analyze(target)).floor()).ceil() as u32;

    let hack_sec_inc = ns.hack_analyze_security(hack_threads, None);
    let hack_weaken_threads = weaken_analyze_threads(ns, hack_sec_inc, cores);

    let money_after_hack = money - hack_amount;
    let grow_multiplier = money / money_after_hack;
    let grow_threads: u32 = ns.growth_analyze(target, grow_multiplier, cores).ceil() as u32;

    let grow_sec_inc = ns.growth_analyze_security(grow_threads, None, cores);
    let grow_weaken_threads = weaken_analyze_threads(ns, grow_sec_inc, cores);
    BatchThreads {
        hack: hack_threads,
        hack_weaken: hack_weaken_threads,
        grow: grow_threads,
        grow_weaken: grow_weaken_threads,
    }
}
