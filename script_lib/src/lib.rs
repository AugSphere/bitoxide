use bitburner_api::NS;

mod f64_total;
pub use f64_total::*;

pub(crate) mod simple_channel;

mod run;
pub use run::*;

pub mod find_servers;
pub mod thread_search;

mod weaken_all;
pub use weaken_all::weaken_all;

pub fn try_root(ns: &NS, host: &str) -> bool {
    let hack_level = ns.get_hacking_level();
    if hack_level < ns.get_server_required_hacking_level(host) {
        return false;
    }
    if ns.brute_ssh(host).is_err() {
        return false;
    }
    if ns.ftpcrack(host).is_err() {
        return false;
    }
    if ns.nuke(host).is_err() {
        return false;
    }
    true
}

pub fn max_money_rate(ns: &NS, host: &str) -> f64 {
    let hack_time = ns.get_hack_time(host);
    let hack_prop = ns.hack_analyze(host);
    let max_money = ns.get_server_max_money(host);
    max_money * hack_prop / hack_time
}
