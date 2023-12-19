//! Bindings for the [Netscript interface](NS).

mod shims;

/// Collection of all functions passed to scripts.
///
/// # Basic usage example
/// ```rust
/// #[wasm_bindgen]
/// pub async fn main_rs(ns: &bitburner_api::NS) {
///     // Basic ns functions can be accessed on the ns object
///     ns.get_hostname();
///     // Some related functions are gathered under a sub-property of the ns object
///     ns.stock.get_price();
///     // Most functions that return a promise need to be awaited.
///     ns.hack('n00dles').await;
/// }
/// ```
pub use shims::NS;
pub use shims::{Arg, BasicHGWOptions};

impl NS {
    /// Arguments passed into the script.
    ///
    /// # Examples
    /// ```rust
    /// // hello.rs
    /// #[wasm_bindgen]
    /// pub fn main_rs(ns: &NS) {
    ///    let args: Vec<Arg> = ns.args();
    ///    ns.tprint(&format!("Hello, world! I said {args:?}"));
    /// }
    /// ```
    /// ```text
    /// [home /]> run hello.js 7 text true
    /// Running script with 1 thread(s), pid 17 and args: [7,"text",true].
    /// hello.js: Hello, world! I said [F64(7.0), String("text"), Bool(true)]
    /// ```
    pub fn args(self: &NS) -> Vec<Arg> {
        shims::parse_args(self.args_shim()).unwrap()
    }

    /// The current script's PID.
    pub fn pid(self: &NS) -> f64 {
        self.pid_shim()
    }

    /// Steal a server's money.
    ///
    /// **RAM cost: 0.1 GB**
    ///
    /// Function that is used to try and hack servers to steal money and gain
    /// hacking experience. The runtime for this command depends on your
    /// hacking level and the target server’s security level when this
    /// function is called. In order to hack a server you must first gain root
    /// access to that server and also have the required hacking level.
    ///
    /// Returns a promise that resolves to the amount of money stolen (which is
    /// zero if the hack is unsuccessful).
    ///
    /// A script can hack a server from anywhere. It does not need to be running
    /// on the same server to hack that server. For example, you can create
    /// a script that hacks the `foodnstuff` server and run that script on
    /// any server in the game.
    ///
    /// A successful `hack()` on a server will raise that server’s security
    /// level by 0.002.
    ///
    /// # Panics
    /// Will panic if JS Promise resolves to something other than [`f64`].
    ///
    /// Invalid host or [`BasicHGWOptions`] can also lead to a panic, for
    /// example if more threads are requested than are available.
    /// **Bitburner is seemingly not able to kill the script in this case,
    /// or catch the exception**. For this reason you should validate the
    /// arguments before calling [`NS::hack`], [`NS::grow`], or
    /// [`NS::weaken`].
    ///
    /// # Examples
    /// ```rust
    /// #[wasm_bindgen]
    /// pub async fn main_rs(ns: &NS) {
    ///     unsafe {
    ///         let amount = ns.hack("foodnstuff", None).await;
    ///         ns.print(&format!("Got {amount}"));
    ///     }
    /// }
    /// ```
    /// ```text
    /// hack: Executing on 'foodnstuff' in 43.022 seconds (t=1)
    /// hack: Failed to hack 'foodnstuff'. Gained 1.500 exp (t=1)
    /// Got 0.0
    /// Script finished running
    /// ```
    pub async unsafe fn hack(self: &NS, host: &str, opts: Option<BasicHGWOptions>) -> f64 {
        self.hack_shim(host, opts).await.unchecked_into_f64()
    }

    /// Spoof money in a server's bank account, increasing the amount available.
    ///
    /// **RAM cost: 0.15 GB**
    ///
    /// Use your hacking skills to increase the amount of money available on a
    /// server.
    ///
    /// Returns the total effective multiplier that was applied to the server's
    /// money ([`f64`]) (after both additive and multiplicative growth).
    ///
    /// Once the grow is complete, $1 is added to the server's available money
    /// for every script thread. This additive growth allows for rescuing a
    /// server even after it is emptied.
    ///
    /// After this addition, the thread count is also used to determine a
    /// multiplier, which the server's money is then multiplied by.
    ///
    /// The multiplier scales exponentially with thread count, and its base
    /// depends on the server's security level and in inherent "growth"
    /// statistic that varies between different servers.
    ///
    /// [`NS::get_server_growth`] can be used to check the inherent growth
    /// statistic of a server.
    ///
    /// [`NS::growth_analyze`] can be used to determine the number of threads
    /// needed for a specified multiplicative portion of server growth.
    ///
    /// To determine the effect of a single grow, obtain access to the Formulas
    /// API and use [`HackingFormulas::growPercent`], or invert
    /// [`NS::growth_analyze`].
    ///
    /// Like [`NS::hack`], [`NS::grow`] can be called on any hackable server,
    /// regardless of where the script is running. Hackable servers are any
    /// servers not owned by the player.
    ///
    /// The `grow()` command requires root access to the target server, but
    /// there is no required hacking level to run the command. It also
    /// raises the security level of the target server based on the number of
    /// threads. The security increase can be determined using
    /// [`NS::growth_analyze_security`].
    ///
    /// # Panics
    /// Will panic if JS Promise resolves to something other than [`f64`].
    ///
    /// Invalid host or [`BasicHGWOptions`] can also lead to a panic, for
    /// example if more threads are requested than are available.
    /// **Bitburner is seemingly not able to kill the script in this case,
    /// or catch the exception**. For this reason you should validate the
    /// arguments before calling [`NS::hack`], [`NS::grow`], or
    /// [`NS::weaken`].
    pub async unsafe fn grow(self: &NS, host: &str, opts: Option<BasicHGWOptions>) -> f64 {
        self.grow_shim(host, opts).await.unchecked_into_f64()
    }

    /// Reduce a server's security level.
    ///
    /// **RAM cost: 0.15 GB**
    ///
    /// Use your hacking skills to attack a server’s security, lowering the
    /// server’s security level. The runtime for this function depends on
    /// your hacking level and the target server’s security level when this
    /// function is called. This function lowers the security level of the
    /// target server by 0.05.
    ///
    /// Returns a promise that resolves to the value by which security was
    /// reduced.
    ///
    /// Like [`NS::hack`] and [`NS::grow`], [`NS::weaken`] can be called on any
    /// server, regardless of where the script is running. This function
    /// requires root access to the target server, but there is no required
    /// hacking level to run the function.
    ///
    /// # Panics
    /// Will panic if JS Promise resolves to something other than [`f64`].
    ///
    /// Invalid host or [`BasicHGWOptions`] can also lead to a panic, for
    /// example if more threads are requested than are available.
    /// **Bitburner is seemingly not able to kill the script in this case,
    /// or catch the exception**. For this reason you should validate the
    /// arguments before calling [`NS::hack`], [`NS::grow`], or
    /// [`NS::weaken`].
    pub async unsafe fn weaken(self: &NS, host: &str, opts: Option<BasicHGWOptions>) -> f64 {
        self.weaken_shim(host, opts).await.unchecked_into_f64()
    }

    /// Predict the effect of weaken.
    ///
    /// **RAM cost: 1 GB**
    ///
    /// Returns the security decrease that would occur if a weaken with this
    /// many threads happened.
    ///
    /// # Arguments
    /// * threads - Amount of threads that will be used.
    /// * cores - Optional. The number of cores of the server that would run
    ///   weaken.
    pub fn weaken_analyze(self: &NS, threads: u32, cores: Option<u32>) -> f64 {
        self.weaken_analyze_shim(threads, cores)
            .unchecked_into_f64()
    }

    /// Get the part of money stolen with a single thread.
    ///
    /// **RAM cost: 1 GB**
    ///
    /// Returns the part of the specified server’s money you will steal with a
    /// single thread hack.
    ///
    /// Like other basic hacking analysis functions, this calculation uses the
    /// current status of the player and server. To calculate using
    /// hypothetical server or player status, obtain access to the Formulas
    /// API and use [`HackingFormulas::hackPercent`].
    ///
    /// # Examples
    /// ```rust
    /// // For example, assume the following returns 0.01:
    /// ns.hack_analyze("foodnstuff")
    /// ```
    /// This means that if hack the foodnstuff server using a single thread,
    /// then you will steal 1%, or 0.01 of its total money.
    /// If you hack using N threads, then you will steal N*0.01 times its total
    /// money.
    pub fn hack_analyze(self: &NS, host: &str) -> f64 {
        self.hack_analyze_shim(host).unchecked_into_f64()
    }

    /// Get the security increase for a number of threads.
    ///
    /// **RAM cost: 1 GB**
    ///
    /// Returns the security increase that would occur if a hack with this many
    /// threads happened.
    ///
    /// # Arguments
    /// * threads - Amount of threads that will be used.
    /// * hostname - Hostname of the target server. The number of threads is
    ///   limited to the number needed to hack the server's maximum amount of
    ///   money.
    pub fn hack_analyze_security(self: &NS, threads: u32, hostname: Option<&str>) -> f64 {
        self.hack_analyze_security_shim(threads, hostname)
            .unchecked_into_f64()
    }

    /// Get the chance of successfully hacking a server.
    ///
    /// **RAM cost: 1 GB**
    ///
    /// Returns the chance you have of successfully hacking the specified
    /// server.
    ///
    /// This returned value is in decimal form, not percentage.
    ///
    /// Like other basic hacking analysis functions, this calculation uses the
    /// current status of the player and server. To calculate using
    /// hypothetical server or player status, obtain access to the Formulas API
    /// and use [`HackingFormulas::hackChance`].
    ///
    /// # Arguments
    /// * host - Hostname of the target server.
    pub fn hack_analyze_chance(self: &NS, host: &str) -> f64 {
        self.hack_analyze_chance_shim(host).unchecked_into_f64()
    }

    /// Calculate the number of grow threads needed for a given multiplicative
    /// growth factor.
    ///
    /// **RAM cost: 1 GB**
    ///
    /// This function returns the total decimal number of [`NS::grow`] threads
    /// needed in order to multiply the money available on the specified
    /// server by a given multiplier, if all threads are executed at the
    /// server's current security level, regardless of how many threads are
    /// assigned to each call.
    ///
    /// Note that there is also an additive factor that is applied before the
    /// multiplier. Each [`NS::grow`] call will add $1 to the host's money
    /// for each thread before applying the multiplier for its thread count.
    /// This means that at extremely low starting money, fewer threads would
    /// be needed to apply the same effective multiplier than
    /// what is calculated by growthAnalyze.
    ///
    /// Like other basic hacking analysis functions, this calculation uses the
    /// current status of the player and server. To calculate using
    /// hypothetical server or player status, obtain access to the Formulas API
    /// and use [`HackingFormulas::growThreads`].
    ///
    /// # Examples
    /// ```rust
    /// // calculate number of grow threads to apply 2x growth multiplier on n00dles (does not include the additive growth).
    /// let grow_threads = ns.growth_analyze("n00dles", 2.0);
    /// ```
    /// # Arguments
    /// * host - Hostname of the target server.
    /// * multiplier - Multiplier that will be applied to a server's money after
    ///   applying additive growth. Decimal form.
    /// * cores - Number of cores on the host running the grow function.
    ///   Optional, defaults to 1.
    pub fn growth_analyze(self: &NS, host: &str, multiplier: f64, cores: Option<u32>) -> f64 {
        self.growth_analyze_shim(host, multiplier, cores)
            .unchecked_into_f64()
    }

    /// Calculate the security increase for a number of grow threads.
    ///
    /// **RAM cost: 1 GB**
    ///
    /// Returns the security increase that would occur if a grow with this many
    /// threads happened.
    ///
    /// # Arguments
    /// * threads - Amount of threads that will be used.
    /// * hostname - Optional. Hostname of the target server. If provided,
    ///   security increase is limited by the number of threads needed to reach
    ///   maximum money.
    /// * cores - Optional. The number of cores of the server that would run
    ///   grow.
    pub fn growth_analyze_security(
        self: &NS,
        threads: u32,
        hostname: Option<&str>,
        cores: Option<u32>,
    ) -> f64 {
        self.growth_analyze_security_shim(threads, hostname, cores)
            .unchecked_into_f64()
    }

    /// Suspends the script for `millis` milliseconds.
    ///
    /// # Examples
    /// ```rust
    /// // This will count from 1 to 10 in your terminal, with one number every 5 seconds
    /// for i in 1..=10 {
    ///     ns.tprint(&i.to_string());
    ///     ns.sleep(5000.0).await;
    /// }
    /// ```
    pub async fn sleep(self: &NS, millis: f64) {
        let Some(ret) = self.sleep_shim(millis).await.as_bool() else {
            panic!("JS ns.sleep Promise did not resolve to a bool");
        };
        assert!(ret, "JS ns.sleep Promise did not resolve to `true`")
    }

    /// Prints one or more values or variables to the script’s logs.
    ///
    /// If the argument is a string, you can color code your message by
    /// prefixing your string with one of these strings:
    ///
    /// - `"ERROR"`: The whole string will be printed in red. Use this prefix to
    ///   indicate that an error has occurred.
    ///
    /// - `"SUCCESS"`: The whole string will be printed in green, similar to the
    ///   default theme of the Terminal. Use this prefix to indicate that
    ///   something is correct.
    ///
    /// - `"WARN"`: The whole string will be printed in yellow. Use this prefix
    ///   to indicate that you or a user of your script should be careful of
    ///   something.
    ///
    /// - `"INFO"`: The whole string will be printed in purplish blue. Use this
    ///   prefix to remind yourself or a user of your script of something. Think
    ///   of this prefix as indicating an FYI (for your information).
    ///
    /// For custom coloring, use ANSI escape sequences. The examples below use
    /// the Unicode escape code `\u{001b}`.
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
    pub fn print(self: &NS, to_print: &str) {
        self.print_shim(to_print)
    }

    /// Prints a string to the Terminal.
    ///
    /// See [`NS::print`] for how to add color to your printed strings.
    pub fn tprint(self: &NS, to_print: &str) {
        self.tprint_shim(to_print)
    }

    /// Get the list of servers connected to a server.
    ///
    /// **RAM cost: 0.2 GB**
    ///
    /// Returns a [`Vec`] containing the hostnames of all servers that are one
    /// node way from the specified target server. If specified host does not
    /// exist, returns [`None`].
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
    pub fn scan(self: &NS, host: Option<&str>) -> Option<Vec<String>> {
        self.scan_shim(host).ok()
    }
}
