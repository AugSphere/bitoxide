use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

use crate::netscript::{NetscriptPort, RunningScript};

/// An argument passed into a script. For use with [`NS::args`].
#[derive(Debug, PartialEq, Clone)]
pub enum Arg {
    Bool(bool),
    F64(f64),
    String(String),
}

impl From<Arg> for JsValue {
    fn from(value: Arg) -> Self {
        match value {
            Arg::Bool(flag) => JsValue::from_bool(flag),
            Arg::F64(n) => JsValue::from_f64(n),
            Arg::String(s) => JsValue::from_str(&s),
        }
    }
}

impl TryFrom<JsValue> for Arg {
    type Error = String;

    fn try_from(value: JsValue) -> Result<Self, Self::Error> {
        if let Some(bool) = value.as_bool() {
            return Ok(Arg::Bool(bool));
        };
        if let Some(float) = value.as_f64() {
            return Ok(Arg::F64(float));
        };
        if let Some(string) = value.as_string() {
            return Ok(Arg::String(string));
        };
        Err(format!("Unexpected argument type of value: {value:?}"))
    }
}

pub trait AsJsExt<O>
where
    O: Into<JsValue>,
{
    fn as_js(self) -> O;
}

impl<T> AsJsExt<js_sys::Array> for Vec<T>
where
    T: Into<JsValue>,
{
    fn as_js(self) -> js_sys::Array {
        js_sys::Array::from_iter(self.into_iter().map(|arg| arg.into()))
    }
}

/// Options to affect the behavior of [`NS::hack`], [`NS::grow`], and
/// [`NS::weaken`].
#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq, Clone, Copy)]
#[wasm_bindgen]
pub struct BasicHGWOptions {
    /// Number of threads to use for this function.
    /// Must be less than or equal to the number of threads the script is
    /// running with.
    pub threads: Option<u32>,
    /// Set to true this action will affect the stock market.
    pub stock: Option<bool>,
    /// Number of additional milliseconds that will be spent waiting between the
    /// start of the function and when it completes.
    pub additionalMsec: Option<f64>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ThreadOrOptions {
    Threads(u32),
    Options(RunOptions),
}

impl From<ThreadOrOptions> for JsValue {
    fn from(value: ThreadOrOptions) -> Self {
        match value {
            ThreadOrOptions::Threads(threads) => threads.into(),
            ThreadOrOptions::Options(options) => options.into(),
        }
    }
}

#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq, Clone, Copy)]
#[wasm_bindgen]
pub struct RunOptions {
    /// Number of threads that the script will run with, defaults to 1.
    pub threads: Option<u32>,

    /// Whether this script is excluded from saves, defaults to false.
    pub temporary: Option<bool>,

    /// The RAM allocation to launch each thread of the script with.
    ///
    /// Lowering this will *not* automatically let you get away with
    /// using less RAM: the dynamic RAM check enforces that all [`NS`]
    /// functions actually called incur their cost. However, if you
    /// know that certain functions that are statically present (and thus
    /// included in the static RAM cost) will never be called in a
    /// particular circumstance, you can use this to avoid paying for
    /// them.
    ///
    /// You can also use this to *increase* the RAM if the static RAM
    /// checker has missed functions that you need to call.
    ///
    /// Must be greater-or-equal to the base RAM cost. Defaults to the
    /// statically calculated cost.
    pub ramOverride: Option<f64>,

    /// Should we fail to run if another instance is running with the exact
    /// same arguments? This used to be the default behavior, now
    /// defaults to false.
    pub preventDuplicates: Option<bool>,
}

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
    pub(super) fn weaken_analyze_shim(this: &NS, threads: u32, cores: Option<u32>) -> JsValue;

    #[wasm_bindgen(method, js_name = hackAnalyze)]
    pub(super) fn hack_analyze_shim(this: &NS, host: &str) -> JsValue;

    #[wasm_bindgen(method, js_name = hackAnalyzeSecurity)]
    pub(super) fn hack_analyze_security_shim(
        this: &NS,
        threads: u32,
        hostname: Option<&str>,
    ) -> JsValue;

    #[wasm_bindgen(method, js_name = hackAnalyzeChance)]
    pub(super) fn hack_analyze_chance_shim(this: &NS, host: &str) -> JsValue;

    #[wasm_bindgen(method, js_name = growthAnalyze)]
    pub(super) fn growth_analyze_shim(
        this: &NS,
        host: &str,
        multiplier: f64,
        cores: Option<u32>,
    ) -> JsValue;

    #[wasm_bindgen(method, js_name = growthAnalyzeSecurity)]
    pub(super) fn growth_analyze_security_shim(
        this: &NS,
        threads: u32,
        hostname: Option<&str>,
        cores: Option<u32>,
    ) -> JsValue;

    #[wasm_bindgen(method, js_name = sleep)]
    pub(super) async fn sleep_shim(this: &NS, millis: f64) -> JsValue;

    #[wasm_bindgen(method, js_name = print)]
    pub(super) fn print_shim(this: &NS, to_print: &str);

    #[wasm_bindgen(method, js_name = tprint)]
    pub(super) fn tprint_shim(this: &NS, to_print: &str);

    #[wasm_bindgen(method, js_name = clearLog)]
    pub(super) fn clear_log_shim(this: &NS);

    #[wasm_bindgen(catch, method, variadic, js_name = tail)]
    pub(super) fn tail_shim(
        this: &NS,
        filename: &JsValue,
        host: Option<&str>,
        args: &JsValue,
    ) -> Result<(), JsValue>;

    #[wasm_bindgen(catch, method, js_name = scan)]
    pub(super) fn scan_shim(this: &NS, host: Option<&str>) -> Result<Vec<String>, JsValue>;

    #[wasm_bindgen(catch, method, js_name = nuke)]
    pub(super) fn nuke_shim(this: &NS, host: &str) -> Result<(), JsValue>;

    #[wasm_bindgen(catch, method, js_name = brutessh)]
    pub(super) fn brutessh_shim(this: &NS, host: &str) -> Result<(), JsValue>;

    #[wasm_bindgen(catch, method, js_name = ftpcrack)]
    pub(super) fn ftpcrack_shim(this: &NS, host: &str) -> Result<(), JsValue>;

    #[wasm_bindgen(catch, method, js_name = relaysmtp)]
    pub(super) fn relaysmtp_shim(this: &NS, host: &str) -> Result<(), JsValue>;

    #[wasm_bindgen(catch, method, js_name = httpworm)]
    pub(super) fn httpworm_shim(this: &NS, host: &str) -> Result<(), JsValue>;

    #[wasm_bindgen(catch, method, js_name = sqlinject)]
    pub(super) fn sqlinject_shim(this: &NS, host: &str) -> Result<(), JsValue>;

    #[wasm_bindgen(method, variadic, js_name = run)]
    pub(super) fn run_shim(
        this: &NS,
        script: &str,
        thread_or_options: &JsValue,
        args: &JsValue,
    ) -> u32;

    #[wasm_bindgen(method, js_name = hasRootAccess)]
    pub(super) fn has_root_access_shim(this: &NS, host: &str) -> bool;

    #[wasm_bindgen(method, js_name = getHackingLevel)]
    pub(super) fn get_hacking_level_shim(this: &NS) -> JsValue;

    #[wasm_bindgen(method, js_name = getServerMoneyAvailable)]
    pub(super) fn get_server_money_available_shim(this: &NS, host: &str) -> f64;

    #[wasm_bindgen(method, js_name = getServerMaxMoney)]
    pub(super) fn get_server_max_money_shim(this: &NS, host: &str) -> f64;

    #[wasm_bindgen(method, js_name = getServerGrowth)]
    pub(super) fn get_server_growth_shim(this: &NS, host: &str) -> f64;

    #[wasm_bindgen(method, js_name = getServerSecurityLevel)]
    pub(super) fn get_server_security_level_shim(this: &NS, host: &str) -> f64;

    #[wasm_bindgen(method, js_name = getServerMinSecurityLevel)]
    pub(super) fn get_server_min_security_level_shim(this: &NS, host: &str) -> f64;

    #[wasm_bindgen(method, js_name = getServerBaseSecurityLevel)]
    pub(super) fn get_server_base_security_level_shim(this: &NS, host: &str) -> f64;

    #[wasm_bindgen(method, js_name = getServerRequiredHackingLevel)]
    pub(super) fn get_server_required_hacking_level_shim(this: &NS, host: &str) -> JsValue;

    #[wasm_bindgen(method, js_name = getServerNumPortsRequired)]
    pub(super) fn get_server_num_ports_required_shim(this: &NS, host: &str) -> JsValue;

    #[wasm_bindgen(method, js_name = serverExists)]
    pub(super) fn server_exists_shim(this: &NS, host: &str) -> bool;

    #[wasm_bindgen(method, js_name = getPortHandle)]
    pub(super) fn get_port_handle_shim(this: &NS, port_number: u32) -> NetscriptPort;

    #[wasm_bindgen(method, variadic, js_name = getRunningScript)]
    pub(super) fn get_running_script_shim(
        this: &NS,
        filename: &JsValue,
        hostname: Option<&str>,
        args: &JsValue,
    ) -> Option<RunningScript>;

    #[wasm_bindgen(method, js_name = getHackTime)]
    pub(super) fn get_hack_time_shim(this: &NS, host: &str) -> f64;

    #[wasm_bindgen(method, js_name = getGrowTime)]
    pub(super) fn get_grow_time_shim(this: &NS, host: &str) -> f64;

    #[wasm_bindgen(method, js_name = getWeakenTime)]
    pub(super) fn get_weaken_time_shim(this: &NS, host: &str) -> f64;
}

pub(super) fn parse_args(object: Vec<JsValue>) -> Result<Vec<Arg>, String> {
    object.into_iter().map(|arg| arg.try_into()).collect()
}
