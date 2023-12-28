use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

use crate::netscript::Arg;

impl RunningScript {
    /// Arguments the script was called with.
    pub fn args(self: &RunningScript) -> Vec<Arg> {
        super::shims::parse_args(self.args_shim()).unwrap()
    }

    /// Filename of the script.
    pub fn filename(self: &RunningScript) -> String {
        self.filename_shim()
    }

    /// Script logs as an array. The newest log entries are at the bottom.
    /// Timestamps, if enabled, are placed inside \[brackets\] at the start
    /// of each line.
    pub fn logs(self: &RunningScript) -> Vec<String> {
        self.logs_shim()
    }

    /// Total amount of hacking experience earned from this script when offline.
    pub fn offline_exp_gained(self: &RunningScript) -> f64 {
        self.offline_exp_gained_shim()
    }

    /// Total amount of money made by this script when offline.
    pub fn offline_money_made(self: &RunningScript) -> f64 {
        self.offline_money_made_shim()
    }

    /// Number of seconds that the script has been running offline.
    pub fn offline_running_time(self: &RunningScript) -> f64 {
        self.offline_running_time_shim()
    }

    /// Total amount of hacking experience earned from this script when online.
    pub fn online_exp_gained(self: &RunningScript) -> f64 {
        self.online_exp_gained_shim()
    }

    /// Total amount of money made by this script when online.
    pub fn online_money_made(self: &RunningScript) -> f64 {
        self.online_money_made_shim()
    }

    /// Number of seconds that this script has been running online
    pub fn online_running_time(self: &RunningScript) -> f64 {
        self.online_running_time_shim()
    }

    /// Process ID.
    pub fn pid(self: &RunningScript) -> u32 {
        self.pid_shim()
    }

    /// How much RAM this script uses for ONE thread.
    pub fn ram_usage(self: &RunningScript) -> f64 {
        self.ram_usage_shim()
    }

    /// Hostname of the server on which this script runs.
    pub fn server(self: &RunningScript) -> String {
        self.server_shim()
    }

    /// Properties of the tail window, or [`None`] if it is not shown.
    pub fn tail_properties(self: &RunningScript) -> Option<TailProperties> {
        self.tail_properties_shim()
    }

    /// The title, as shown in the script's log box. Defaults to the name +
    /// args, but can be changed by the user.
    pub fn title(self: &RunningScript) -> String {
        self.title_shim()
    }

    /// Number of threads that this script runs with.
    pub fn threads(self: &RunningScript) -> u32 {
        self.threads_shim()
    }

    /// Whether this RunningScript is excluded from saves.
    pub fn temporary(self: &RunningScript) -> bool {
        self.temporary_shim()
    }
}

#[wasm_bindgen]
extern "C" {
    pub type RunningScript;

    #[wasm_bindgen(method, getter, js_name = args)]
    fn args_shim(this: &RunningScript) -> Vec<JsValue>;

    #[wasm_bindgen(method, getter, js_name = filename)]
    fn filename_shim(this: &RunningScript) -> String;

    #[wasm_bindgen(method, getter, js_name = logs)]
    fn logs_shim(this: &RunningScript) -> Vec<String>;

    #[wasm_bindgen(method, getter, js_name = offlineExpGained)]
    fn offline_exp_gained_shim(this: &RunningScript) -> f64;

    #[wasm_bindgen(method, getter, js_name = offlineMoneyMade)]
    fn offline_money_made_shim(this: &RunningScript) -> f64;

    #[wasm_bindgen(method, getter, js_name = offlineRunningTime)]
    fn offline_running_time_shim(this: &RunningScript) -> f64;

    #[wasm_bindgen(method, getter, js_name = onlineExpGained)]
    fn online_exp_gained_shim(this: &RunningScript) -> f64;

    #[wasm_bindgen(method, getter, js_name = onlineMoneyMade)]
    fn online_money_made_shim(this: &RunningScript) -> f64;

    #[wasm_bindgen(method, getter, js_name = onlineRunningTime)]
    fn online_running_time_shim(this: &RunningScript) -> f64;

    #[wasm_bindgen(method, getter, js_name = pid)]
    fn pid_shim(this: &RunningScript) -> u32;

    #[wasm_bindgen(method, getter, js_name = ramUsage)]
    fn ram_usage_shim(this: &RunningScript) -> f64;

    #[wasm_bindgen(method, getter, js_name = server)]
    fn server_shim(this: &RunningScript) -> String;

    #[wasm_bindgen(method, getter, js_name = tailProperties)]
    fn tail_properties_shim(this: &RunningScript) -> Option<TailProperties>;

    #[wasm_bindgen(method, getter, js_name = title)]
    fn title_shim(this: &RunningScript) -> String;

    #[wasm_bindgen(method, getter, js_name = threads)]
    fn threads_shim(this: &RunningScript) -> u32;

    #[wasm_bindgen(method, getter, js_name = temporary)]
    fn temporary_shim(this: &RunningScript) -> bool;

    pub type TailProperties;

    /// X-coordinate of the log window
    #[wasm_bindgen(method, getter)]
    pub fn x(this: &TailProperties) -> f64;

    /// Y-coordinate of the log window
    #[wasm_bindgen(method, getter)]
    pub fn y(this: &TailProperties) -> f64;

    /// Width of the log window content area
    #[wasm_bindgen(method, getter)]
    pub fn width(this: &TailProperties) -> f64;

    /// Height of the log window content area
    #[wasm_bindgen(method, getter)]
    pub fn height(this: &TailProperties) -> f64;
}
