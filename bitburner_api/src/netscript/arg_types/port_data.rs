use wasm_bindgen::JsValue;

/// Type for data sent through Netscript ports.
#[derive(Debug, PartialEq, Clone)]
pub enum PortData {
    Number(f64),
    String(String),
}

impl From<PortData> for JsValue {
    fn from(value: PortData) -> Self {
        match value {
            PortData::Number(n) => JsValue::from_f64(n),
            PortData::String(string) => JsValue::from_str(&string),
        }
    }
}

impl TryFrom<JsValue> for PortData {
    type Error = String;

    fn try_from(value: JsValue) -> Result<Self, Self::Error> {
        if let Some(float) = value.as_f64() {
            return Ok(PortData::Number(float));
        };
        if let Some(string) = value.as_string() {
            return Ok(PortData::String(string));
        };
        Err(format!("Unexpected argument type of value: {value:?}"))
    }
}
