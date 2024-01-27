use bitburner_api::{wasm_bindgen, wasm_bindgen_futures, NS};

use script_lib::web_sys;

#[wasm_bindgen]
pub fn main_rs(ns: &NS) {
    let window = web_sys::window().expect("should have a window in this context");
    let performance = window
        .performance()
        .expect("performance should be available");
    let now = performance.now();
    ns.tprint(&format!("{now:?}"))
}
