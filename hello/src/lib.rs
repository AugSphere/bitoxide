use bitburner_api::parse_args;
use bitburner_api::wasm_bindgen;
use bitburner_api::Arg;

#[wasm_bindgen]
pub fn main_rs(ns: &bitburner_api::NS) {
    let args: Vec<Arg> = parse_args(ns.args()).unwrap();

    let mut buffer = String::new();
    buffer += &format!("Hello, world! I said {:?}", args);
    ns.tprint(&buffer);
}
