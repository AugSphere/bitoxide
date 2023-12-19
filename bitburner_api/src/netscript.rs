//! Bindings for the [Netscript interface](NS)
#![deny(rustdoc::all)]
use wasm_bindgen::{prelude::*, JsValue};

#[wasm_bindgen]
extern "C" {
    /// Collection of all functions passed to scripts.
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
    /// Returns a [`Vec`] of [`JsValue`]s that can be parsed to [`Arg`]s with [`parse_args`].
    ///
    /// # Examples
    /// ```rust
    /// // hello.rs
    /// #[wasm_bindgen]
    /// pub fn main_rs(ns: &NS) {
    ///     let args: Vec<Arg> = parse_args(ns.args()).unwrap();
    ///     ns.tprint(&format!("Hello, world! I said {:?}", args));
    /// }
    /// ```
    /// ```text
    /// [home /]> run hello.js 7 text true
    /// Running script with 1 thread(s), pid 17 and args: [7,"text",true].
    /// hello.js: Hello, world! I said [F64(7.0), String("text"), Bool(true)]
    /// ```
    #[wasm_bindgen(method, getter)]
    pub fn args(this: &NS) -> Vec<JsValue>;

    /// The current script's PID.
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
    /// Returns a promise that resolves to the amount of money stolen ([`f64`]) (which is zero if the hack is unsuccessful).
    ///
    /// A script can hack a server from anywhere. It does not need to be running on the same
    /// server to hack that server. For example, you can create a script that hacks the `foodnstuff`
    /// server and run that script on any server in the game.
    ///
    /// A successful `hack()` on a server will raise that server’s security level by 0.002.
    ///
    /// # Examples
    /// ```rust
    /// #[wasm_bindgen]
    /// pub async fn main_rs(ns: &NS) {
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
    /// Returns a promise that resolves to the value by which security was reduced ([`f64`]).
    ///
    /// Like [`NS::hack`] and [`NS::grow`], [`NS::weaken`] can be called on any server, regardless of
    /// where the script is running. This function requires root access to the target server, but
    /// there is no required hacking level to run the function.
    ///
    /// # Examples
    /// ```rust
    /// let current_security = ns.getServerSecurityLevel("foodnstuff");
    /// current_security -= ns.weaken("foodnstuff").await;
    /// ```
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
    /// Returns the total effective multiplier that was applied to the server's money ([`f64`]) (after both additive and multiplicative growth).
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
    /// [`NS::getServerGrowth`] can be used to check the inherent growth statistic of a server.
    ///
    /// [`NS::growthAnalyze`] can be used to determine the number of threads needed for a specified
    /// multiplicative portion of server growth.
    ///
    /// To determine the effect of a single grow, obtain access to the Formulas API and use
    /// [`HackingFormulas::growPercent`], or invert [`NS::growthAnalyze`].
    ///
    /// Like [`NS::hack`], [`NS::grow`] can be called on any hackable server, regardless of where the script is
    /// running. Hackable servers are any servers not owned by the player.
    ///
    /// The `grow()` command requires root access to the target server, but there is no required hacking
    /// level to run the command. It also raises the security level of the target server based on the number of threads.
    /// The security increase can be determined using [`NS::growthAnalyzeSecurity`].
    ///
    /// # Examples
    /// ```rust
    /// let current_money = ns.getServerMoneyAvailable("n00dles");
    /// currentMoney *= ns.grow("foodnstuff").await;
    /// ```
    #[wasm_bindgen(catch, method)]
    pub async fn grow(
        this: &NS,
        host: String,
        opts: Option<BasicHGWOptions>,
    ) -> Result<JsValue, JsValue>;

    /// Suspends the script for `millis` milliseconds.
    /// # Examples
    /// ```rust
    /// // This will count from 1 to 10 in your terminal, with one number every 5 seconds
    /// for i in 1..=10 {
    ///     ns.tprint(&i.to_string());
    ///     ns.sleep(5000.0).await;
    /// }
    /// ```
    #[wasm_bindgen(method)]
    pub async fn sleep(this: &NS, millis: f64);

    /// Prints one or more values or variables to the script’s logs.
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
    /// escape code `\u{001b}`.
    ///
    /// # Examples
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
    /// let cyan = "\u{001b}[36m";
    /// let green = "\u{001b}[32m";
    /// let red = "\u{001b}[31m";
    /// let reset = "\u{001b}[0m";
    /// ns.print(&format!("{red}Ugh! What a mess.{reset}"));
    /// ns.print(&format!("{green}Well done!{reset}"));
    /// ns.print(&format!("{cyan}ERROR Should this be in red?{reset}"));
    /// ```
    #[wasm_bindgen(method)]
    pub fn print(this: &NS, print: &str);

    /// Prints a string to the Terminal.
    ///
    /// See [`NS::print`] for how to add color to your printed strings.
    #[wasm_bindgen(method)]
    pub fn tprint(this: &NS, print: &str);

    /// Get the list of servers connected to a server.
    ///
    /// **RAM cost: 0.2 GB**
    ///
    /// Returns a [`Vec`] containing the hostnames of all servers that are one
    /// node way from the specified target server.
    ///
    /// # Examples
    /// ```rust
    /// // All servers that are one hop from the current server.
    /// ns.tprint("Neighbors of current server.");
    /// let neighbors = ns.scan(None).unwrap();
    /// for neighbor in neighbors {
    ///     ns.tprint(&neighbor);
    /// }
    /// // All neighbors of n00dles.
    /// const TARGET: &str = "n00dles";
    /// let neighbors = ns.scan(Some(TARGET)).unwrap();
    /// ns.tprint(&format!("Neighbors of {TARGET}."));
    /// for neighbor in neighbors {
    ///     ns.tprint(&neighbor);
    /// }
    /// ```
    /// # Errors
    /// Returns the JS exception as a [`JsValue`] on being called for a non-existent host.
    ///
    /// # Arguments
    /// * host - Optional. Hostname of the server to scan, default to current server.
    #[wasm_bindgen(catch, method)]
    pub fn scan(this: &NS, host: Option<&str>) -> Result<Vec<String>, JsValue>;

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

/// Options to affect the behavior of [`NS::hack`], [`NS::grow`], and [`NS::weaken`].
#[allow(non_snake_case)]
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
    pub additionalMsec: Option<f64>,
}

/// An argument passed into a script. For use with [`NS::args`].
#[derive(Debug, PartialEq)]
pub enum Arg {
    Bool(bool),
    F64(f64),
    String(String),
}

/// A helper to make the output of [`NS::args`] typed.
/// # Errors
/// If passed [`JsValue`]s that are not of the correct type, will return an error message that contains a
/// [`Debug`](trait@core::fmt::Debug) representation of the first value that fails to parse.
pub fn parse_args(object: Vec<JsValue>) -> Result<Vec<Arg>, String> {
    object
        .into_iter()
        .map(|val| {
            if let Some(bool) = val.as_bool() {
                return Ok(Arg::Bool(bool));
            };
            if let Some(float) = val.as_f64() {
                return Ok(Arg::F64(float));
            };
            if let Some(string) = val.as_string() {
                return Ok(Arg::String(string));
            };
            Err(format!("Unexpected argument type of value: {:?}", val))
        })
        .collect()
}
