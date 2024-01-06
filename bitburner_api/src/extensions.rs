use wasm_bindgen::JsValue;

pub trait AsJsExt<Output>
where
    Output: Into<JsValue>,
{
    fn as_js(self) -> Output;
}

impl<T> AsJsExt<js_sys::Array> for Vec<T>
where
    T: Into<JsValue>,
{
    fn as_js(self) -> js_sys::Array {
        js_sys::Array::from_iter(self.into_iter().map(|arg| arg.into()))
    }
}
