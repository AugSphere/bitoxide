use wasm_bindgen::JsValue;

pub trait ToJsExt<Output>
where
    Output: Into<JsValue>,
{
    fn to_js(self) -> Output;
}

impl<T> ToJsExt<js_sys::Array> for Vec<T>
where
    T: Into<JsValue>,
{
    fn to_js(self) -> js_sys::Array {
        js_sys::Array::from_iter(self.into_iter().map(|arg| arg.into()))
    }
}
