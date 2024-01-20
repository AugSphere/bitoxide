use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

/// Options for [`run`](crate::netscript::NS::run) and
/// [`exec`](crate::netscript::NS::exec).
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ThreadOrOptions {
    Threads(u32),
    Options(RunOptions),
}

impl From<u32> for ThreadOrOptions {
    fn from(value: u32) -> Self {
        ThreadOrOptions::Threads(value)
    }
}

impl From<RunOptions> for ThreadOrOptions {
    fn from(value: RunOptions) -> Self {
        ThreadOrOptions::Options(value)
    }
}

#[allow(non_snake_case)]
#[derive(Debug, Default, PartialEq, Clone, Copy)]
#[wasm_bindgen]
pub struct RunOptions {
    /// Number of threads that the script will run with, defaults to 1.
    pub threads: Option<u32>,

    /// Whether this script is excluded from saves, defaults to false.
    pub temporary: Option<bool>,

    /// The RAM allocation to launch each thread of the script with.
    ///
    /// Lowering this will *not* automatically let you get away with
    /// using less RAM: the dynamic RAM check enforces that all
    /// [`NS`](crate::netscript::NS) functions actually called incur their
    /// cost. However, if you know that certain functions that are
    /// statically present (and thus included in the static RAM cost) will
    /// never be called in a particular circumstance, you can use this to
    /// avoid paying for them.
    ///
    /// You can also use this to *increase* the RAM if the static RAM
    /// checker has missed functions that you need to call.
    ///
    /// Must be greater-or-equal to the base RAM cost. Defaults to the
    /// statically calculated cost.
    pub ramOverride: Option<f64>,

    /// Should we fail to run if another instance is running with the exact
    /// same arguments? This used to be the default behavior, now
    /// defaults to false.
    pub preventDuplicates: Option<bool>,
}

impl From<ThreadOrOptions> for JsValue {
    fn from(value: ThreadOrOptions) -> Self {
        match value {
            ThreadOrOptions::Threads(threads) => threads.into(),
            ThreadOrOptions::Options(options) => options.into(),
        }
    }
}
