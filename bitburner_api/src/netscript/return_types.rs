use wasm_bindgen::JsValue;

/// Runtime error captured from Bitburner.
///
/// # Examples
/// Usually results from passing in a host that does not exist:
/// ```rust
/// # use bitburner_api::wasm_bindgen;
/// #[wasm_bindgen]
/// pub fn main_rs(ns: &bitburner_api::NS) {
///     match ns.get_running_script(Some("deadbeef".into()), None, vec![]) {
///         Ok(_) => {}
///         Err(msg) => ns.print(&msg.to_string()),
///     }
/// }
/// ```
/// The script above will print to log:
/// ```text
/// RUNTIME ERROR
/// error_example.js@home (PID - 31)
///
/// getRunningScript: Invalid filename, was not a valid path: deadbeef
///
/// Stack:
/// error_example.js:L-1@unknown
/// error_example.js:L597@handleError
/// error_example.js:L872@imports.wbg.__wbg_getRunningScript_5ddf4d5f394d93ce
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BitburnerError {
    msg: String,
}

impl BitburnerError {
    pub fn new(msg: &str) -> Self {
        BitburnerError {
            msg: msg.to_owned(),
        }
    }
}

impl From<JsValue> for BitburnerError {
    fn from(value: JsValue) -> Self {
        BitburnerError {
            msg: value.as_string().unwrap(),
        }
    }
}

impl From<BitburnerError> for String {
    fn from(value: BitburnerError) -> Self {
        value.msg
    }
}

impl From<String> for BitburnerError {
    fn from(value: String) -> Self {
        BitburnerError { msg: value }
    }
}

impl std::fmt::Display for BitburnerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.msg.fmt(f)
    }
}

impl std::error::Error for BitburnerError {}
