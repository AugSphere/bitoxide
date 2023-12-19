//! Bindings for Bitburner functions.
//! Thank you, github.com/paulcdejean
#![deny(rustdoc::all)]

pub extern crate js_sys;
pub extern crate wasm_bindgen;
pub extern crate wasm_bindgen_futures;
pub use wasm_bindgen::{prelude::*, JsValue};

pub mod netscript;
pub use netscript::NS;
