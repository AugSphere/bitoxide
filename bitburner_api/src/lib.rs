pub extern crate js_sys;
pub extern crate wasm_bindgen;
pub extern crate wasm_bindgen_futures;
pub use wasm_bindgen::{prelude::*, JsValue};

// thank you github.com/paulcdejean
#[wasm_bindgen]
extern "C" {
    // Continue adding more imported structs from Bitburner and their associated
    // methods in here.
    //
    // For object attributes, skip to after this `extern` block.

    /// Collection of all functions passed to scripts
    ///
    /// # Basic usage example
    /// ```rust
    /// #[wasm_bindgen]
    /// pub async fn main_rs(ns: &bitburner_api::NS) {
    ///     // Basic ns functions can be accessed on the ns object
    ///     ns.getHostname();
    ///     // Some related functions are gathered under a sub-property of the ns object
    ///     ns.stock.getPrice();
    ///     // Most functions that return a promise need to be awaited.
    ///     ns.hack('n00dles').await;
    /// }
    /// ```
    pub type NS;

    /// Arguments passed into the script.
    ///
    /// Returns a [Vec] of [JsValue]s that can be parsed to [Args] with [parse_args].
    ///
    /// # Example
    /// ```rust
    /// // hello.rs
    /// #[wasm_bindgen]
    /// pub fn main_rs(ns: &bitburner_api::NS) {
    ///     let args: Vec<Args> = parse_args(ns.args()).unwrap();
    ///
    ///     let mut buffer = String::new();
    ///     buffer += &format!("Hello, world! I said {:?}", args);
    ///     ns.tprint(&buffer);
    /// }
    /// ```
    /// ```text
    /// [home /]> run hello.js 7 text true
    /// Running script with 1 thread(s), pid 17 and args: [7,"text",true].
    /// hello.js: Hello, world! I said [F64(7.0), String("text"), Bool(true)]
    /// ```
    #[wasm_bindgen(method, getter)]
    pub fn args(this: &NS) -> Vec<JsValue>;

    /// The current script's PID
    #[wasm_bindgen(method, getter)]
    pub fn pid(this: &NS) -> f64;

    /// Steal a server's money.
    ///
    /// **RAM cost: 0.1 GB**
    ///
    /// Function that is used to try and hack servers to steal money and gain hacking experience.
    /// The runtime for this command depends on your hacking level and the target server’s
    /// security level when this function is called. In order to hack a server you must first gain root access to that server
    /// and also have the required hacking level.
    ///
    /// A script can hack a server from anywhere. It does not need to be running on the same
    /// server to hack that server. For example, you can create a script that hacks the `foodnstuff`
    /// server and run that script on any server in the game.
    ///
    /// A successful `hack()` on a server will raise that server’s security level by 0.002.
    ///
    /// Returns a promise that resolves to the amount of money stolen ([f64]) (which is zero if the hack is unsuccessful).
    /// # Example
    /// ```rust
    /// #[wasm_bindgen]
    /// pub async fn main_rs(ns: &bitburner_api::NS) {
    ///     let amount = ns.hack("foodnstuff".to_owned(), None).await;
    ///     ns.print(&format!("Got {:?}", amount.unchecked_into_f64()));
    /// }
    /// ```
    /// ```text
    /// hack: Executing on 'foodnstuff' in 43.022 seconds (t=1)
    /// hack: Failed to hack 'foodnstuff'. Gained 1.500 exp (t=1)
    /// Got 0.0
    /// Script finished running
    /// ```
    #[wasm_bindgen(catch, method)]
    pub async fn hack(
        this: &NS,
        host: String,
        opts: Option<BasicHGWOptions>,
    ) -> Result<JsValue, JsValue>;

    /// Reduce a server's security level.
    ///
    /// **RAM cost: 0.15 GB**
    ///
    /// Use your hacking skills to attack a server’s security, lowering the server’s security level.
    /// The runtime for this function depends on your hacking level and the target server’s security
    /// level when this function is called. This function lowers the security level of the target server by 0.05.
    ///
    /// Like [hack](NS::hack) and [grow](NS::grow), [weaken](NS::weaken) can be called on any server, regardless of
    /// where the script is running. This function requires root access to the target server, but
    /// there is no required hacking level to run the function.
    ///
    /// # Example
    /// ```rust
    /// let current_security = ns.getServerSecurityLevel("foodnstuff");
    /// current_security -= ns.weaken("foodnstuff").await;
    /// ```
    ///
    /// Returns a promise that resolves to the value by which security was reduced ([f64]).
    #[wasm_bindgen(catch, method)]
    pub async fn weaken(
        this: &NS,
        host: String,
        opts: Option<BasicHGWOptions>,
    ) -> Result<JsValue, JsValue>;

    /// Spoof money in a server's bank account, increasing the amount available.
    ///
    /// **RAM cost: 0.15 GB**
    ///
    /// Use your hacking skills to increase the amount of money available on a server.
    ///
    /// Once the grow is complete, $1 is added to the server's available money for every script thread. This additive
    /// growth allows for rescuing a server even after it is emptied.
    ///
    /// After this addition, the thread count is also used to determine a multiplier, which the server's money is then
    /// multiplied by.
    ///
    /// The multiplier scales exponentially with thread count, and its base depends on the server's security
    /// level and in inherent "growth" statistic that varies between different servers.
    ///
    /// [getServerGrowth](NS::getServerGrowth) can be used to check the inherent growth statistic of a server.
    ///
    /// [growthAnalyze](NS::growthAnalyze) can be used to determine the number of threads needed for a specified
    /// multiplicative portion of server growth.
    ///
    /// To determine the effect of a single grow, obtain access to the Formulas API and use
    /// [HackingFormulas::growPercent], or invert [growthAnalyze](NS::growthAnalyze).
    ///
    /// Like [hack](NS::hack), [grow](NS::grow) can be called on any hackable server, regardless of where the script is
    /// running. Hackable servers are any servers not owned by the player.
    ///
    /// The grow() command requires root access to the target server, but there is no required hacking
    /// level to run the command. It also raises the security level of the target server based on the number of threads.
    /// The security increase can be determined using [growthAnalyzeSecurity](NS::growthAnalyzeSecurity).
    ///
    /// # Example
    /// ```rust
    /// let current_money = ns.getServerMoneyAvailable("n00dles");
    /// currentMoney *= ns.grow("foodnstuff").await;
    /// ```
    ///
    /// Returns the total effective multiplier that was applied to the server's money ([f64]) (after both additive and multiplicative growth).
    #[wasm_bindgen(catch, method)]
    pub async fn grow(
        this: &NS,
        host: String,
        opts: Option<BasicHGWOptions>,
    ) -> Result<JsValue, JsValue>;

    /// Prints one or more values or variables to the script’s logs.
    /// RAM cost: 0 GB
    ///
    /// If the argument is a string, you can color code your message by prefixing your
    /// string with one of these strings:
    ///
    /// - `"ERROR"`: The whole string will be printed in red. Use this prefix to indicate
    ///   that an error has occurred.
    ///
    /// - `"SUCCESS"`: The whole string will be printed in green, similar to the default
    ///   theme of the Terminal. Use this prefix to indicate that something is correct.
    ///
    /// - `"WARN"`: The whole string will be printed in yellow. Use this prefix to
    ///   indicate that you or a user of your script should be careful of something.
    ///
    /// - `"INFO"`: The whole string will be printed in purplish blue. Use this prefix to
    ///   remind yourself or a user of your script of something. Think of this prefix as
    ///   indicating an FYI (for your information).
    ///
    /// For custom coloring, use ANSI escape sequences. The examples below use the Unicode
    /// escape code `\u001b`. The color coding also works if `\u001b` is replaced with
    /// the hexadecimal escape code `\x1b`. The Bash escape code `\e` is not supported.
    /// The octal escape code `\033` is not allowed because the game runs JavaScript in
    /// strict mode.
    ///
    /// # Example
    /// ```rust
    /// // Default color coding.
    /// ns.print("ERROR means something's wrong.");
    /// ns.print("SUCCESS means everything's OK.");
    /// ns.print("WARN Tread with caution!");
    /// ns.print("WARNING, warning, danger, danger!");
    /// ns.print("WARNing! Here be dragons.");
    /// ns.print("INFO for your I's only (FYI).");
    /// ns.print("INFOrmation overload!");
    /// // Custom color coding.
    /// let cyan = "\u001b[36m";
    /// let green = "\u001b[32m";
    /// let red = "\u001b[31m";
    /// let reset = "\u001b[0m";
    /// ns.print("{red}Ugh! What a mess.{reset}");
    /// ns.print("{green}Well done!{reset}");
    /// ns.print("{cyan}ERROR Should this be in red?{reset}");
    /// ns.tail();
    /// ```
    #[wasm_bindgen(method)]
    pub fn print(this: &NS, print: &str);

    #[wasm_bindgen(method)]
    pub fn tprint(this: &NS, print: &str);

    #[wasm_bindgen(method)]
    pub fn scan(this: &NS, scan: Option<&str>) -> Vec<JsValue>;

    #[wasm_bindgen(catch, method)]
    pub fn nuke(this: &NS, host: &str) -> Result<(), JsValue>;

    #[wasm_bindgen(catch, method)]
    pub fn brutessh(this: &NS, hostname: &str) -> Result<(), JsValue>;

    #[wasm_bindgen(catch, method)]
    pub fn ftpcrack(this: &NS, hostname: &str) -> Result<(), JsValue>;

    #[wasm_bindgen(catch, method)]
    pub fn relaysmtp(this: &NS, hostname: &str) -> Result<(), JsValue>;

    #[wasm_bindgen(catch, method)]
    pub fn httpworm(this: &NS, hostname: &str) -> Result<(), JsValue>;

    #[wasm_bindgen(catch, method)]
    pub fn sqlinject(this: &NS, hostname: &str) -> Result<(), JsValue>;

    #[wasm_bindgen(method)]
    pub fn getServer(this: &NS, host: Option<&str>) -> Server;

    pub type Server;
}

/// Options to affect the behavior of [NS::hack], [NS::grow], and [NS::weaken].
#[derive(Debug, Default)]
#[wasm_bindgen]
pub struct BasicHGWOptions {
    /// Number of threads to use for this function.
    /// Must be less than or equal to the number of threads the script is running with.
    pub threads: Option<u8>,
    /// Set to true this action will affect the stock market.
    pub stock: Option<bool>,
    /// Number of additional milliseconds that will be spent waiting between the start of the function and when it
    /// completes.
    pub additional_msec: Option<f64>,
}

/// Arguments passed into the script.
#[derive(Debug, PartialEq)]
pub enum Args {
    Bool(bool),
    F64(f64),
    String(String),
}

pub fn parse_args(object: Vec<JsValue>) -> Result<Vec<Args>, String> {
    object
        .into_iter()
        .map(|val| {
            if let Some(bool) = val.as_bool() {
                return Ok(Args::Bool(bool));
            };
            if let Some(float) = val.as_f64() {
                return Ok(Args::F64(float));
            };
            if let Some(string) = val.as_string() {
                return Ok(Args::String(string));
            };
            Err(format!("Unexpected argument type of value: {:?}", val))
        })
        .collect()
}
