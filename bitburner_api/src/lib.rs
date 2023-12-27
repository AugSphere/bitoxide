//! Bindings for Bitburner functions.
//!
//! Thank you, [github.com/paulcdejean](https://github.com/paulcdejean)
//!
//! Created from the definitions of Bitburner v2.5.2.
//!
//! Note that some functions have different signatures from their Bitburner
//! counterparts. For example [`NS::sleep`] does not have a return value (in
//! Bitburner it always returns true).
//!
//! It should be assumed all functions may panic on receiving an unexpected type
//! from JS, even if not explicitly mentioned in the docs.
//!
//! # Bitburner "async"
//! Bitburner promises are not real promises, generally the only safe thing to
//! do with them is to immediately await before calling any other netscript
//! functions. This means you can not combine them via join or select, they are
//! in essence blocking functions. As a consequence of this, error handling gets
//! messed up due to Bitburner promises not fulfilling the invariants expected
//! of JS promises. This leads to the following:
//!
//! ## **All async functions can hang Bitburner scripts!**
//! Any errors from Bitburner async functions can lead to the scripts being
//! stuck without Bitburner being able to automatically kill them and propagate
//! the errors. These functions are marked unsafe. There are safe variants that
//! do runtime checks which increase the amount of (in-game RAM) they use. If
//! you use the unsafe variants, make sure the inputs are valid before calling
//! them.

pub extern crate js_sys;
pub extern crate wasm_bindgen;
pub extern crate wasm_bindgen_futures;
pub use wasm_bindgen::prelude::*;
pub use wasm_bindgen::JsValue;

pub mod netscript;
pub use netscript::NS;
