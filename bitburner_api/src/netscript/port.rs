use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

use crate::netscript::PortData;

impl NetscriptPort {
    ///  Write data to a port.
    ///
    /// Returns the data popped off the queue if it was full.
    pub fn write(self: &NetscriptPort, value: PortData) -> Option<PortData> {
        let result = self.write_shim(value.into());
        result.try_into().ok()
    }

    /// Attempt to write data to the port.
    ///
    /// Returns [`true`] if the data was added to the port, [`false`] if the
    /// port was full.
    pub fn try_write(self: &NetscriptPort, value: PortData) -> bool {
        self.try_write_shim(value.into())
    }

    /// Sleeps until the port is written to.
    pub async fn next_write(self: &NetscriptPort) {
        self.next_write_shim().await;
    }

    /// Shift an element out of the port.
    ///
    /// This function will remove the first element from the port and return it.
    /// If the port is empty, then the string `“NULL PORT DATA”` will be
    /// returned.
    ///
    /// Returns the data read.
    pub fn read(self: &NetscriptPort) -> PortData {
        self.read_shim().try_into().unwrap()
    }

    /// Retrieve the first element from the port without removing it.
    ///
    /// This function is used to peek at the data from a port. It returns the
    /// first element in the specified port without removing that element. If
    /// the port is empty, the string `“NULL PORT DATA”` will be returned.
    ///
    /// Returns the data read.
    pub fn peek(self: &NetscriptPort) -> PortData {
        self.peek_shim().try_into().unwrap()
    }

    /// Check if the port is full.
    ///
    /// Returns [`true`] if the port is full, otherwise [`false`].
    pub fn full(self: &NetscriptPort) -> bool {
        self.full_shim()
    }

    /// Check if the port is empty.
    ///
    /// Returns [`true`] if the port is empty, otherwise [`false`].
    pub fn empty(self: &NetscriptPort) -> bool {
        self.empty_shim()
    }

    /// Empties all data from the port.
    pub fn clear(self: &NetscriptPort) {
        self.clear_shim()
    }
}

#[wasm_bindgen]
extern "C" {
    pub type NetscriptPort;

    #[wasm_bindgen(method, js_name = write)]
    fn write_shim(this: &NetscriptPort, value: JsValue) -> JsValue;

    #[wasm_bindgen(method, js_name = tryWrite)]
    fn try_write_shim(this: &NetscriptPort, value: JsValue) -> bool;

    #[wasm_bindgen(method, js_name = nextWrite)]
    async fn next_write_shim(this: &NetscriptPort);

    #[wasm_bindgen(method, js_name = read)]
    fn read_shim(this: &NetscriptPort) -> JsValue;

    #[wasm_bindgen(method, js_name = peek)]
    fn peek_shim(this: &NetscriptPort) -> JsValue;

    #[wasm_bindgen(method, js_name = full)]
    fn full_shim(this: &NetscriptPort) -> bool;

    #[wasm_bindgen(method, js_name = empty)]
    fn empty_shim(this: &NetscriptPort) -> bool;

    #[wasm_bindgen(method, js_name = clear)]
    fn clear_shim(this: &NetscriptPort);
}
