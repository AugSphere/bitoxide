use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

/// An argument passed into a script. For use with [`NS::args`].
#[derive(Debug, PartialEq)]
pub enum Arg {
    Bool(bool),
    F64(f64),
    String(String),
}

/// Options to affect the behavior of [`NS::hack`], [`NS::grow`], and
/// [`NS::weaken`].
#[allow(non_snake_case)]
#[derive(Debug, Default, Clone, Copy)]
#[wasm_bindgen]
pub struct BasicHGWOptions {
    /// Number of threads to use for this function.
    /// Must be less than or equal to the number of threads the script is
    /// running with.
    pub threads: Option<u8>,
    /// Set to true this action will affect the stock market.
    pub stock: Option<bool>,
    /// Number of additional milliseconds that will be spent waiting between the
    /// start of the function and when it completes.
    pub additionalMsec: Option<f64>,
}

#[wasm_bindgen]
extern "C" {
    pub type NS;

    #[wasm_bindgen(method, getter, js_name = args)]
    pub(super) fn args_shim(this: &NS) -> Vec<JsValue>;

    #[wasm_bindgen(method, getter, js_name = pid)]
    pub(super) fn pid_shim(this: &NS) -> f64;

    #[wasm_bindgen(method, js_name = hack)]
    pub(super) async fn hack_shim(this: &NS, host: &str, opts: Option<BasicHGWOptions>) -> JsValue;

    #[wasm_bindgen(method, js_name = grow)]
    pub(super) async fn grow_shim(this: &NS, host: &str, opts: Option<BasicHGWOptions>) -> JsValue;

    #[wasm_bindgen(method, js_name = weaken)]
    pub(super) async fn weaken_shim(
        this: &NS,
        host: &str,
        opts: Option<BasicHGWOptions>,
    ) -> JsValue;

    #[wasm_bindgen(method, js_name = weakenAnalyze)]
    pub(super) fn weaken_analyze_shim(this: &NS, threads: u8, cores: Option<u8>) -> JsValue;

    #[wasm_bindgen(method, js_name = hackAnalyze)]
    pub(super) fn hack_analyze_shim(this: &NS, host: &str) -> JsValue;

    #[wasm_bindgen(method, js_name = sleep)]
    pub(super) async fn sleep_shim(this: &NS, millis: f64) -> JsValue;

    #[wasm_bindgen(method, js_name = print)]
    pub(super) fn print_shim(this: &NS, to_print: &str);

    #[wasm_bindgen(method, js_name = tprint)]
    pub(super) fn tprint_shim(this: &NS, to_print: &str);

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
}

pub(super) fn parse_args(object: Vec<JsValue>) -> Result<Vec<Arg>, String> {
    object
        .into_iter()
        .map(|val| {
            if let Some(bool) = val.as_bool() {
                return Ok(Arg::Bool(bool));
            };
            if let Some(float) = val.as_f64() {
                return Ok(Arg::F64(float));
            };
            if let Some(string) = val.as_string() {
                return Ok(Arg::String(string));
            };
            Err(format!("Unexpected argument type of value: {val:?}"))
        })
        .collect()
}
