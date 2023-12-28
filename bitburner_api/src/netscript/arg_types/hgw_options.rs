use wasm_bindgen::prelude::*;

/// Options to affect the behavior of [`hack`](crate::netscript::NS::hack),
/// [`grow`](crate::netscript::NS::grow), and
/// [`weaken`](crate::netscript::NS::weaken).
#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq, Clone, Copy)]
#[wasm_bindgen]
pub struct BasicHGWOptions {
    /// Number of threads to use for this function.
    /// Must be less than or equal to the number of threads the script is
    /// running with.
    pub threads: Option<u32>,
    /// Set to true this action will affect the stock market.
    pub stock: Option<bool>,
    /// Number of additional milliseconds that will be spent waiting between the
    /// start of the function and when it completes.
    pub additionalMsec: Option<f64>,
}
