use wasm_bindgen::JsValue;

use crate::netscript::BitburnerError;

/// An argument passed into a script. For use with
/// [`args`](crate::netscript::NS::args).
#[derive(Debug, PartialEq, Clone)]
pub enum Arg {
    Bool(bool),
    F64(f64),
    String(String),
}

/// Type for identifying scripts by either id or filename in
/// [`get_running_script`](crate::netscript::NS::get_running_script)
#[derive(Debug, PartialEq, Clone)]
pub enum FilenameOrPID {
    Pid(u32),
    Name(String),
}

impl From<u32> for FilenameOrPID {
    fn from(value: u32) -> Self {
        FilenameOrPID::Pid(value)
    }
}

impl From<&str> for FilenameOrPID {
    fn from(value: &str) -> Self {
        FilenameOrPID::Name(value.to_owned())
    }
}

impl From<bool> for Arg {
    fn from(value: bool) -> Self {
        Arg::Bool(value)
    }
}

impl From<f64> for Arg {
    fn from(value: f64) -> Self {
        Arg::F64(value)
    }
}

impl From<&str> for Arg {
    fn from(value: &str) -> Self {
        Arg::String(value.to_owned())
    }
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
    type Error = BitburnerError;

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
        Err(format!("Unexpected argument type of value: {value:?}").into())
    }
}

impl From<FilenameOrPID> for JsValue {
    fn from(value: FilenameOrPID) -> Self {
        match value {
            FilenameOrPID::Pid(pid) => JsValue::from_f64(pid.into()),
            FilenameOrPID::Name(string) => JsValue::from_str(&string),
        }
    }
}
