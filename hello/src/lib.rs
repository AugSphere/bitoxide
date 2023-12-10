use bitoxide;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn main_rs(ns: &bitoxide::NS) {
    let mut buffer = "Hello, world! I said".to_owned();
    let args = bitoxide::get_attribute(ns, "args", |a| Some(js_sys::Array::from(a)))
        .unwrap()
        .unwrap();
    let args_iter = args.iter().map(|a| a.as_string().unwrap());
    buffer += &(" ".to_owned() + &args_iter.collect::<Vec<String>>().join(" "));
    ns.tprint(&buffer);
}
