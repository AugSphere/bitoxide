//! Bindings for Bitburner functions.
//!
//! Thank you, [github.com/paulcdejean](https://github.com/paulcdejean)
//!
//! Created from the definitions of Bitburner v2.5.1.
//!
//! Note that some functions have different signatures from their Bitburner counterparts.
//! For example [`NS::sleep`] does not have a return value (in Bitburner it always returns true).
#![deny(rustdoc::all)]

pub extern crate js_sys;
pub extern crate wasm_bindgen;
pub extern crate wasm_bindgen_futures;
pub use wasm_bindgen::{prelude::*, JsValue};

pub mod netscript;
pub use netscript::NS;
