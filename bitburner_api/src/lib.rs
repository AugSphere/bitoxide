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
//! functions. This means you can not combine them via join, select, or even
//! register a handler with functions like [`js_sys::Promise::then2`]. They are
//! in practice blocking functions. As a consequence of this, error handling
//! involving [`netscript`] "async" functions is prone to panics,
//! which leads to the following:
//!
//! ## **All async functions can hang Bitburner scripts!**
//! Any errors from Bitburner async functions can lead to the scripts being
//! stuck without Bitburner being able to automatically kill them and propagate
//! the errors. These functions are marked unsafe. If
//! you use them, make sure the inputs are valid before calling
//! them and await the future **immediately**, without passing it to any
//! intermediate handling.
//!
//! ## An alternative to Bitburner async
//! You can genrally avoid having to call any "async" functions of
//! [`netscript`]. [gloo-timers](https://crates.io/crates/gloo-timers)
//! (among many others) has a usable alternative to [`NS::sleep`], and
//! [`NS::run`] can be used to launch helper scripts like
//! ```js
//! export async function main(ns) {
//!   await ns.hack(ns.args[0]);
//! }
//! ```
//! without blocking.

pub extern crate js_sys;
pub extern crate wasm_bindgen;
pub extern crate wasm_bindgen_futures;
pub use wasm_bindgen::prelude::*;
pub use wasm_bindgen::JsValue;

pub mod netscript;
pub use netscript::NS;

mod extensions;
