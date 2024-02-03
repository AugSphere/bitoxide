use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

use crate::netscript::{
    Arg, BasicHGWOptions, BitburnerError, NetscriptPort, RunningScript, Server,
};

#[wasm_bindgen]
extern "C" {
    pub type NS;

    #[wasm_bindgen(method, getter, js_name = args)]
    pub(super) fn args_shim(this: &NS) -> Vec<JsValue>;

    #[wasm_bindgen(method, getter, js_name = pid)]
    pub(super) fn pid_shim(this: &NS) -> f64;

    #[wasm_bindgen(method, js_name = hack)]
    pub(super) async unsafe fn hack_shim(
        this: &NS,
        host: &str,
        opts: Option<BasicHGWOptions>,
    ) -> JsValue;

    #[wasm_bindgen(method, js_name = grow)]
    pub(super) async unsafe fn grow_shim(
        this: &NS,
        host: &str,
        opts: Option<BasicHGWOptions>,
    ) -> JsValue;

    #[wasm_bindgen(method, js_name = weaken)]
    pub(super) async unsafe fn weaken_shim(
        this: &NS,
        host: &str,
        opts: Option<BasicHGWOptions>,
    ) -> JsValue;

    #[wasm_bindgen(method, js_name = weakenAnalyze)]
    pub(super) fn weaken_analyze_shim(this: &NS, threads: u32, cores: Option<u32>) -> f64;

    #[wasm_bindgen(method, catch, js_name = hackAnalyze)]
    pub(super) fn hack_analyze_shim(this: &NS, host: &str) -> Result<f64, BitburnerError>;

    #[wasm_bindgen(method, catch, js_name = hackAnalyzeSecurity)]
    pub(super) fn hack_analyze_security_shim(
        this: &NS,
        threads: u32,
        hostname: Option<&str>,
    ) -> Result<f64, BitburnerError>;

    #[wasm_bindgen(method, catch, js_name = hackAnalyzeChance)]
    pub(super) fn hack_analyze_chance_shim(this: &NS, host: &str) -> Result<f64, BitburnerError>;

    #[wasm_bindgen(method, catch, js_name = growthAnalyze)]
    pub(super) fn growth_analyze_shim(
        this: &NS,
        host: &str,
        multiplier: f64,
        cores: Option<u32>,
    ) -> Result<f64, BitburnerError>;

    #[wasm_bindgen(method, catch, js_name = growthAnalyzeSecurity)]
    pub(super) fn growth_analyze_security_shim(
        this: &NS,
        threads: u32,
        hostname: Option<&str>,
        cores: Option<u32>,
    ) -> Result<f64, BitburnerError>;

    #[wasm_bindgen(method, js_name = sleep)]
    pub(super) async unsafe fn sleep_shim(this: &NS, millis: f64) -> JsValue;

    #[wasm_bindgen(method, js_name = print)]
    pub(super) fn print_shim(this: &NS, to_print: &str);

    #[wasm_bindgen(method, js_name = tprint)]
    pub(super) fn tprint_shim(this: &NS, to_print: &str);

    #[wasm_bindgen(method, js_name = clearLog)]
    pub(super) fn clear_log_shim(this: &NS);

    #[wasm_bindgen(method, catch, js_name = disableLog)]
    pub(super) fn disable_log_shim(this: &NS, fun: &str) -> Result<(), BitburnerError>;

    #[wasm_bindgen(method, catch, js_name = enableLog)]
    pub(super) fn enable_log_shim(this: &NS, fun: &str) -> Result<(), BitburnerError>;

    #[wasm_bindgen(catch, method, variadic, js_name = tail)]
    pub(super) fn tail_shim(
        this: &NS,
        filename: &JsValue,
        host: Option<&str>,
        args: &JsValue,
    ) -> Result<(), BitburnerError>;

    #[wasm_bindgen(catch, method, js_name = scan)]
    pub(super) fn scan_shim(this: &NS, host: Option<&str>) -> Result<Vec<String>, BitburnerError>;

    #[wasm_bindgen(catch, method, js_name = nuke)]
    pub(super) fn nuke_shim(this: &NS, host: &str) -> Result<(), BitburnerError>;

    #[wasm_bindgen(catch, method, js_name = brutessh)]
    pub(super) fn brutessh_shim(this: &NS, host: &str) -> Result<(), BitburnerError>;

    #[wasm_bindgen(catch, method, js_name = ftpcrack)]
    pub(super) fn ftpcrack_shim(this: &NS, host: &str) -> Result<(), BitburnerError>;

    #[wasm_bindgen(catch, method, js_name = relaysmtp)]
    pub(super) fn relaysmtp_shim(this: &NS, host: &str) -> Result<(), BitburnerError>;

    #[wasm_bindgen(catch, method, js_name = httpworm)]
    pub(super) fn httpworm_shim(this: &NS, host: &str) -> Result<(), BitburnerError>;

    #[wasm_bindgen(catch, method, js_name = sqlinject)]
    pub(super) fn sqlinject_shim(this: &NS, host: &str) -> Result<(), BitburnerError>;

    #[wasm_bindgen(method, catch, variadic, js_name = run)]
    pub(super) fn run_shim(
        this: &NS,
        script: &str,
        thread_or_options: &JsValue,
        args: &JsValue,
    ) -> Result<u32, BitburnerError>;

    #[wasm_bindgen(method, js_name = kill)]
    pub(super) fn kill_shim(this: &NS, pid: u32) -> bool;

    #[wasm_bindgen(method, catch, js_name = hasRootAccess)]
    pub(super) fn has_root_access_shim(this: &NS, host: &str) -> Result<bool, BitburnerError>;

    #[wasm_bindgen(method, js_name = getHostname)]
    pub(super) fn get_hostname_shim(this: &NS) -> String;

    #[wasm_bindgen(method, js_name = getHackingLevel)]
    pub(super) fn get_hacking_level_shim(this: &NS) -> u32;

    #[wasm_bindgen(method, catch, js_name = getServer)]
    pub(super) fn get_server_shim(this: &NS, host: Option<&str>) -> Result<Server, BitburnerError>;

    #[wasm_bindgen(method, catch, js_name = getServerMoneyAvailable)]
    pub(super) fn get_server_money_available_shim(
        this: &NS,
        host: &str,
    ) -> Result<f64, BitburnerError>;

    #[wasm_bindgen(method, catch, js_name = getServerMaxMoney)]
    pub(super) fn get_server_max_money_shim(this: &NS, host: &str) -> Result<f64, BitburnerError>;

    #[wasm_bindgen(method, catch, js_name = getServerGrowth)]
    pub(super) fn get_server_growth_shim(this: &NS, host: &str) -> Result<f64, BitburnerError>;

    #[wasm_bindgen(method, catch, js_name = getServerSecurityLevel)]
    pub(super) fn get_server_security_level_shim(
        this: &NS,
        host: &str,
    ) -> Result<f64, BitburnerError>;

    #[wasm_bindgen(method, catch, js_name = getServerMinSecurityLevel)]
    pub(super) fn get_server_min_security_level_shim(
        this: &NS,
        host: &str,
    ) -> Result<f64, BitburnerError>;

    #[wasm_bindgen(method, catch, js_name = getServerBaseSecurityLevel)]
    pub(super) fn get_server_base_security_level_shim(
        this: &NS,
        host: &str,
    ) -> Result<f64, BitburnerError>;

    #[wasm_bindgen(method, catch, js_name = getServerMaxRam)]
    pub(super) fn get_server_max_ram_shim(this: &NS, host: &str) -> Result<f64, BitburnerError>;

    #[wasm_bindgen(method, catch, js_name = getServerUsedRam)]
    pub(super) fn get_server_used_ram_shim(this: &NS, host: &str) -> Result<f64, BitburnerError>;

    #[wasm_bindgen(method, catch, js_name = getServerRequiredHackingLevel)]
    pub(super) fn get_server_required_hacking_level_shim(
        this: &NS,
        host: &str,
    ) -> Result<u32, BitburnerError>;

    #[wasm_bindgen(method, catch, js_name = getServerNumPortsRequired)]
    pub(super) fn get_server_num_ports_required_shim(
        this: &NS,
        host: &str,
    ) -> Result<u32, BitburnerError>;

    #[wasm_bindgen(method, js_name = serverExists)]
    pub(super) fn server_exists_shim(this: &NS, host: &str) -> bool;

    #[wasm_bindgen(method, catch, js_name = getPortHandle)]
    pub(super) fn get_port_handle_shim(
        this: &NS,
        port_number: u32,
    ) -> Result<NetscriptPort, BitburnerError>;

    #[wasm_bindgen(method, catch, variadic, js_name = isRunning)]
    pub(super) fn is_running_shim(
        this: &NS,
        script: &JsValue,
        host: Option<&str>,
        args: &JsValue,
    ) -> Result<bool, BitburnerError>;

    #[wasm_bindgen(method, catch, variadic, js_name = getRunningScript)]
    pub(super) fn get_running_script_shim(
        this: &NS,
        filename: &JsValue,
        hostname: Option<&str>,
        args: &JsValue,
    ) -> Result<Option<RunningScript>, BitburnerError>;

    #[wasm_bindgen(method, catch, js_name = getHackTime)]
    pub(super) fn get_hack_time_shim(this: &NS, host: &str) -> Result<f64, BitburnerError>;

    #[wasm_bindgen(method, catch, js_name = getGrowTime)]
    pub(super) fn get_grow_time_shim(this: &NS, host: &str) -> Result<f64, BitburnerError>;

    #[wasm_bindgen(method, catch, js_name = getWeakenTime)]
    pub(super) fn get_weaken_time_shim(this: &NS, host: &str) -> Result<f64, BitburnerError>;
}

pub(super) fn parse_args(object: Vec<JsValue>) -> Result<Vec<Arg>, BitburnerError> {
    object.into_iter().map(|arg| arg.try_into()).collect()
}
