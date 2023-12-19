use bitburner_api::netscript::{parse_args, Arg};
use bitburner_api::wasm_bindgen;

#[wasm_bindgen]
pub fn main_rs(ns: &bitburner_api::NS) {
    let args: Vec<Arg> = parse_args(ns.args()).unwrap();
    ns.tprint(&format!("Hello, world! I said {:?}", args));
}
