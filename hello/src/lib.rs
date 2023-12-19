use bitburner_api::netscript::Arg;
use bitburner_api::wasm_bindgen;

#[wasm_bindgen]
pub fn main_rs(ns: &bitburner_api::NS) {
    let args: Vec<Arg> = ns.args();
    ns.tprint(&format!("Hello, world! I said {args:?}"));
}
