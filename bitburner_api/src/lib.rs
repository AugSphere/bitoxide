//! Bindings for Bitburner functions.
//!
//! Thank you, [github.com/paulcdejean](https://github.com/paulcdejean)
//!
//! Created from the definitions of Bitburner v2.5.1.
//!
//! Note that some functions have different signatures from their Bitburner counterparts.
//! For example [`NS::sleep`] does not have a return value (in Bitburner it always returns true).
//!
//! It should be assumed all functions may panic on receiving an unexpected type from JS, even if
//! not explicitly mentioned in the docs.
//!
//! # All async functions can hang Bitburner scripts!
//! Invalid inputs to async functions can lead to the scripts being stuck without
//! Bitburner being able to automatically kill them and propagate the errors.
//! For this reason they are marked unsafe. Make sure to validate inputs before calling them.

pub extern crate js_sys;
pub extern crate wasm_bindgen;
pub extern crate wasm_bindgen_futures;
pub use wasm_bindgen::{prelude::*, JsValue};

pub mod netscript;
pub use netscript::NS;
