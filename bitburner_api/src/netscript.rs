//! Bindings for the [Netscript interface](NS).

mod arg_types;
mod port;
mod return_types;
mod running_script;
mod server;
mod shims;

pub use arg_types::{Arg, BasicHGWOptions, FilenameOrPID, PortData, RunOptions, ThreadOrOptions};
/// Object representing a port. A port is a serialized queue. Output of
/// [`get_port_handle`](NS::get_port_handle).
///
/// Size of the port queue is controlled by the `Netscript port size` setting in
/// Bitburner options.
pub use port::NetscriptPort;
pub use return_types::BitburnerError;
/// Properties of a script, can be obtained from
/// [`get_running_script`](NS::get_running_script).
pub use running_script::RunningScript;
/// Shape and position of a [`tail`](NS::tail) window.
pub use running_script::TailProperties;
/// A server.
///
/// Not all servers have all of these properties - optional properties
/// are filled with default values.
pub use server::Server;
/// Collection of all Bitburner functions passed to scripts.
pub use shims::NS;

use crate::extensions::ToJsExt;

impl NS {
    /// Arguments passed into the script.
    ///
    /// # Examples
    /// ```rust
    /// // hello.rs
    /// # use bitburner_api::netscript::Arg;
    /// # use bitburner_api::NS;
    /// # use bitburner_api::wasm_bindgen;
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
    pub fn pid(self: &NS) -> u32 {
        self.pid_shim() as u32
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
    /// # Safety
    /// Invalid host or [`BasicHGWOptions`] can will lead to a JS exception that
    /// will hang the script, for example if more threads are requested than
    /// are available. The future not being awaited immediately can have a
    /// similar effect. See [bitburner_api
    /// docs](crate#all-async-functions-can-hang-bitburner-scripts).
    ///
    /// # Examples
    /// ```rust
    /// # use bitburner_api::NS;
    /// # use bitburner_api::wasm_bindgen;
    /// #[wasm_bindgen]
    /// pub async fn main_rs(ns: &NS) {
    ///     unsafe {
    ///         let amount = ns.hack("foodnstuff", None).await;
    ///         ns.tprint(&format!("Got {amount}"));
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
    /// # Safety
    /// Invalid host or [`BasicHGWOptions`] can will lead to a JS exception that
    /// will hang the script, for example if more threads are requested than
    /// are available. The future not being awaited immediately can have a
    /// similar effect. See [bitburner_api
    /// docs](crate#all-async-functions-can-hang-bitburner-scripts).
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
    /// # Safety
    /// Invalid host or [`BasicHGWOptions`] can will lead to a JS exception that
    /// will hang the script, for example if more threads are requested than
    /// are available. The future not being awaited immediately can have a
    /// similar effect. See [bitburner_api
    /// docs](crate#all-async-functions-can-hang-bitburner-scripts).
    pub async unsafe fn weaken(self: &NS, host: &str, opts: Option<BasicHGWOptions>) -> f64 {
        self.weaken_shim(host, opts).await.unchecked_into_f64()
    }

    /// Check arguments common to [`NS::hack`], [`NS::grow`], [`NS::weaken`] for
    /// correctness.
    ///
    /// **RAM cost: 0.45 GB** when [`BasicHGWOptions::threads`] is specified,
    /// **0.15** otherwise
    pub fn check_hgw_args(
        self: &NS,
        host: &str,
        opts: Option<BasicHGWOptions>,
    ) -> Result<(), String> {
        if !self.server_exists(host) {
            let msg = format!("Server {host} does not exist");
            self.print(&("ERROR ".to_owned() + &msg));
            return Err(msg);
        }
        if !self.has_root_access(host)? {
            let msg = format!("No root access to {host}");
            self.print(&("ERROR ".to_owned() + &msg));
            return Err(msg);
        }
        if let Some(BasicHGWOptions {
            threads: Some(requested_threads),
            ..
        }) = opts
        {
            let own_script = self.get_running_script(None, None, self.args())?.unwrap();
            let own_threads = own_script.threads();
            if requested_threads > own_threads {
                let msg =
                    format!("Not enough threads available: requested {requested_threads}, script has access to {own_threads}");
                self.print(&("ERROR ".to_owned() + &msg));
                return Err(msg);
            }
        };
        Ok(())
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
    /// # use bitburner_api::NS;
    /// # use bitburner_api::wasm_bindgen;
    /// #[wasm_bindgen]
    /// pub fn main_rs(ns: &NS) {
    ///     // For example, assume the following returns 0.01:
    ///     ns.hack_analyze("foodnstuff").unwrap();
    /// }
    /// ```
    /// This means that if hack the foodnstuff server using a single thread,
    /// then you will steal 1%, or 0.01 of its total money.
    /// If you hack using N threads, then you will steal N*0.01 times its total
    /// money.
    pub fn hack_analyze(self: &NS, host: &str) -> Result<f64, BitburnerError> {
        self.hack_analyze_shim(host)
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
    pub fn hack_analyze_security(
        self: &NS,
        threads: u32,
        hostname: Option<&str>,
    ) -> Result<f64, BitburnerError> {
        self.hack_analyze_security_shim(threads, hostname)
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
    pub fn hack_analyze_chance(self: &NS, host: &str) -> Result<f64, BitburnerError> {
        self.hack_analyze_chance_shim(host)
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
    /// # use bitburner_api::NS;
    /// # use bitburner_api::wasm_bindgen;
    /// #[wasm_bindgen]
    /// pub fn main_rs(ns: &NS) {
    ///     // calculate number of grow threads to apply 2x growth multiplier on n00dles
    ///     // (does not include the additive growth)
    ///     let grow_threads = ns.growth_analyze("n00dles", 2.0, None).unwrap();
    /// }
    /// ```
    /// # Arguments
    /// * host - Hostname of the target server.
    /// * multiplier - Multiplier that will be applied to a server's money after
    ///   applying additive growth. Decimal form.
    /// * cores - Number of cores on the host running the grow function.
    ///   Optional, defaults to 1.
    pub fn growth_analyze(
        self: &NS,
        host: &str,
        multiplier: f64,
        cores: Option<u32>,
    ) -> Result<f64, BitburnerError> {
        self.growth_analyze_shim(host, multiplier, cores)
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
    ) -> Result<f64, BitburnerError> {
        self.growth_analyze_security_shim(threads, hostname, cores)
    }

    /// Suspends the script for `millis` milliseconds.
    ///
    /// # Examples
    /// ```rust
    /// // This will count from 1 to 10 in your terminal, with one number every 5 seconds
    /// # use bitburner_api::{wasm_bindgen, wasm_bindgen_futures};
    /// #[wasm_bindgen]
    /// pub async fn main_rs(ns: &bitburner_api::NS) {
    ///     for i in 1..=10 {
    ///         ns.tprint(&i.to_string());
    ///         unsafe {
    ///             ns.sleep(5000.0).await;
    ///         }
    ///     }
    /// }
    /// ```
    ///
    /// # Safety
    /// The future not being awaited immediately can cause the Bitburner script
    /// to hang. See [bitburner_api
    /// docs](crate#all-async-functions-can-hang-bitburner-scripts).
    pub async unsafe fn sleep(self: &NS, millis: f64) {
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
    /// # use bitburner_api::NS;
    /// # use bitburner_api::wasm_bindgen;
    /// #[wasm_bindgen]
    /// pub fn main_rs(ns: &NS) {
    ///     // Default color coding.
    ///     ns.print("ERROR means something's wrong.");
    ///     ns.print("SUCCESS means everything's OK.");
    ///     ns.print("WARN Tread with caution!");
    ///     ns.print("WARNING, warning, danger, danger!");
    ///     ns.print("WARNing! Here be dragons.");
    ///     ns.print("INFO for your I's only (FYI).");
    ///     ns.print("INFOrmation overload!");
    ///     // Custom color coding.
    ///     let cyan = "\u{001b}[36m";
    ///     let green = "\u{001b}[32m";
    ///     let red = "\u{001b}[31m";
    ///     let reset = "\u{001b}[0m";
    ///     ns.print(&format!("{red}Ugh! What a mess.{reset}"));
    ///     ns.print(&format!("{green}Well done!{reset}"));
    ///     ns.print(&format!("{cyan}ERROR Should this be in red?{reset}"));
    /// }
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

    /// Clears the script’s logs.
    pub fn clear_log(self: &NS) {
        self.clear_log_shim()
    }

    /// Disables logging for the given function.
    ///
    /// Logging can be disabled for all functions by passing `ALL` as the
    /// argument.
    pub fn disable_log(self: &NS, fun: &str) -> Result<(), BitburnerError> {
        self.disable_log_shim(fun)
    }

    /// Enable logging for a certain function.
    ///
    /// Re-enables logging for the given function. If `ALL` is passed into this
    /// function as an argument, then it will revert the effects of
    /// disableLog(`ALL`).
    pub fn enable_log(self: &NS, fun: &str) -> Result<(), BitburnerError> {
        self.enable_log_shim(fun)
    }

    /// Open the tail window of a script.
    ///
    /// Opens a script’s logs. This is functionally the same as the tail
    /// Terminal command.
    ///
    /// If the function is called with no arguments, it will open the current
    /// script’s logs.
    ///
    /// Otherwise, the PID or filename, hostname/ip, and args… arguments can be
    /// used to get the logs from another script. Remember that scripts are
    /// uniquely identified by both their names and arguments.
    ///
    /// # Arguments
    /// * filename - Optional. Filename or PID of the script being tailed. If
    ///   omitted, the current script is tailed.
    /// * hostname - Optional. Hostname of the script being tailed. Defaults to
    ///   the server this script is running on. If args are specified, this is
    ///   not optional.
    /// * args - Arguments for the script being tailed.
    pub fn tail(
        self: &NS,
        filename: Option<FilenameOrPID>,
        hostname: Option<&str>,
        args: Vec<Arg>,
    ) -> Result<(), BitburnerError> {
        let filename = filename.into();
        let hostname = hostname.map(|s| s.to_owned());
        self.tail_shim(&filename, hostname.as_deref(), &args.to_js())
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
    /// # use bitburner_api::NS;
    /// # use bitburner_api::wasm_bindgen;
    /// #[wasm_bindgen]
    /// pub fn main_rs(ns: &NS) {
    ///     // All servers that are one hop from the current server.
    ///     ns.tprint("Neighbors of current server.");
    ///     let neighbors = ns.scan(None).unwrap();
    ///     for neighbor in neighbors {
    ///         ns.tprint(&neighbor);
    ///     }
    ///     // All neighbors of n00dles.
    ///     const TARGET: &str = "n00dles";
    ///     let neighbors = ns.scan(Some(TARGET)).unwrap();
    ///     ns.tprint(&format!("Neighbors of {TARGET}."));
    ///     for neighbor in neighbors {
    ///         ns.tprint(&neighbor);
    ///     }
    /// }
    /// ```
    pub fn scan(self: &NS, host: Option<&str>) -> Result<Vec<String>, BitburnerError> {
        self.scan_shim(host)
    }

    /// Runs NUKE.exe on a server.
    ///
    /// **RAM cost: 0.05 GB**
    ///
    /// Running NUKE.exe on a target server gives you root access which means
    /// you can execute scripts on said server. NUKE.exe must exist on your home
    /// computer.
    pub fn nuke(self: &NS, host: &str) -> Result<(), BitburnerError> {
        self.nuke_shim(host)
    }

    /// Runs BruteSSH.exe on a server.
    ///
    /// **RAM cost: 0.05 GB**
    ///
    /// Runs the BruteSSH.exe program on the target server. BruteSSH.exe must
    /// exist on your home computer.
    pub fn brute_ssh(self: &NS, host: &str) -> Result<(), BitburnerError> {
        self.brutessh_shim(host)
    }

    /// Runs FTPCrack.exe on a server.
    ///
    /// **RAM cost: 0.05 GB**
    ///
    /// Runs the FTPCrack.exe program on the target server. FTPCrack.exe must
    /// exist on your home computer.
    pub fn ftpcrack(self: &NS, host: &str) -> Result<(), BitburnerError> {
        self.ftpcrack_shim(host)
    }

    /// Start another script on the current server.
    ///
    /// **RAM cost: 1 GB**
    ///
    /// Run a script as a separate process. This function can only be used to
    /// run scripts located on the current server (the server running the
    /// script that calls this function). Requires a significant
    /// amount of RAM to run this command.
    ///
    /// The second argument is either a thread count, or a [`RunOptions`] object
    /// that can specify the number of threads (among other things).
    ///
    /// If the script was successfully started, then this functions returns the
    /// PID of that script. Otherwise, it returns 0.
    ///
    /// PID stands for Process ID. The PID is a unique identifier for each
    /// script. The PID will always be a positive integer.
    pub fn run(
        self: &NS,
        script: &str,
        thread_or_options: Option<ThreadOrOptions>,
        args: Vec<Arg>,
    ) -> Result<u32, BitburnerError> {
        self.run_shim(script, &thread_or_options.into(), &args.to_js())
    }

    /// Terminate the script with the provided PID.
    /// **RAM cost: 0.5 GB**
    ///
    /// Kills the script with the provided PID.
    ///
    /// Returns [`true`] if the script is successfully killed, and [`false`]
    /// otherwise.
    pub fn kill(self: &NS, pid: u32) -> bool {
        self.kill_shim(pid)
    }

    /// Check if you have root access on a server.
    ///
    /// **RAM cost: 0.05 GB**
    ///
    /// Returns a boolean indicating whether or not the player has root access
    /// to the specified target server.
    ///
    /// # Examples
    /// ```rust
    /// # use bitburner_api::NS;
    /// # use bitburner_api::wasm_bindgen;
    /// #[wasm_bindgen]
    /// pub fn main_rs(ns: &NS) {
    ///     if !ns.has_root_access("foodnstuff").unwrap() {
    ///         ns.nuke("foodnstuff").unwrap();
    ///     }
    /// }
    /// ```
    pub fn has_root_access(self: &NS, host: &str) -> Result<bool, BitburnerError> {
        self.has_root_access_shim(host)
    }

    /// Returns a string with the hostname of the server that the script is
    /// running on.
    ///
    /// **RAM cost: 0.05 GB**
    pub fn get_hostname(self: &NS) -> String {
        self.get_hostname_shim()
    }

    /// Returns the player’s current hacking level.
    ///
    /// **RAM cost: 0.05 GB**
    pub fn get_hacking_level(self: &NS) -> u32 {
        self.get_hacking_level_shim()
    }

    /// Returns a server object for the given server. Defaults to the running
    /// script's server if host is not specified.
    ///
    /// **RAM cost: 2 GB**
    pub fn get_server(self: &NS, host: Option<&str>) -> Result<Server, BitburnerError> {
        self.get_server_shim(host)
    }

    /// Get money available on a server.
    ///
    /// **RAM cost: 0.1 GB**
    ///
    /// Returns the amount of money available on a server.
    /// Running this function on the home computer will return the player’s
    /// money.
    pub fn get_server_money_available(self: &NS, host: &str) -> Result<f64, BitburnerError> {
        self.get_server_money_available_shim(host)
    }

    /// Get the maximum money available on a server.
    ///
    /// **RAM cost: 0.1 GB**
    ///
    /// Returns the maximum amount of money that can be available on a server.
    pub fn get_server_max_money(self: &NS, host: &str) -> Result<f64, BitburnerError> {
        self.get_server_max_money_shim(host)
    }

    /// Get a server growth parameter.
    ///
    /// **RAM cost: 0.1 GB**
    ///
    /// Returns the server’s intrinsic “growth parameter”. This growth
    /// parameter is a number typically between 0 and 100 that represents
    /// how quickly the server’s money grows. This parameter affects the
    /// percentage by which the server’s money is increased when using the
    /// grow function. A higher growth parameter will result in a
    /// higher percentage increase from grow.
    pub fn get_server_growth(self: &NS, host: &str) -> Result<f64, BitburnerError> {
        self.get_server_growth_shim(host)
    }

    /// Get server security level.
    ///
    /// **RAM cost: 0.1 GB**
    ///
    /// Returns the security level of the target server. A server’s security
    /// level is denoted by a number, typically between 1 and 100
    /// (but it can go above 100).
    pub fn get_server_security_level(self: &NS, host: &str) -> Result<f64, BitburnerError> {
        self.get_server_security_level_shim(host)
    }

    /// Returns the minimum security level of the target server.
    ///
    /// **RAM cost: 0.1 GB**
    pub fn get_server_min_security_level(self: &NS, host: &str) -> Result<f64, BitburnerError> {
        self.get_server_min_security_level_shim(host)
    }

    /// Get the base security level of a server.
    ///
    /// **RAM cost: 0.1 GB**
    ///
    /// Returns the base security level of the target server.
    /// For the server's actual security level, use
    /// [`NS::get_server_security_level`].
    pub fn get_server_base_security_level(self: &NS, host: &str) -> Result<f64, BitburnerError> {
        self.get_server_base_security_level_shim(host)
    }

    /// Get the maximum amount of RAM (GB) on a server.
    ///
    /// **RAM cost: 0.05 GB**
    pub fn get_server_max_ram(self: &NS, host: &str) -> Result<f64, BitburnerError> {
        self.get_server_max_ram_shim(host)
    }

    /// Get the used RAM (GB) on a server.
    ///
    /// **RAM cost: 0.05 GB**
    pub fn get_server_used_ram(self: &NS, host: &str) -> Result<f64, BitburnerError> {
        self.get_server_used_ram_shim(host)
    }

    /// Returns the required hacking level of the target server.
    ///
    /// **RAM cost: 0.1 GB**
    pub fn get_server_required_hacking_level(self: &NS, host: &str) -> Result<u32, BitburnerError> {
        self.get_server_required_hacking_level_shim(host)
    }

    /// Returns the number of open ports required to successfully run NUKE.exe
    /// on the specified server.
    ///
    /// **RAM cost: 0.1 GB**
    pub fn get_server_num_ports_required(self: &NS, host: &str) -> Result<u32, BitburnerError> {
        self.get_server_num_ports_required_shim(host)
    }

    /// Returns a boolean denoting whether or not the specified server exists.
    ///
    /// **RAM cost: 0.1 GB**
    pub fn server_exists(self: &NS, host: &str) -> bool {
        self.server_exists_shim(host)
    }

    /// Get a handle to a Netscript Port.
    pub fn get_port_handle(self: &NS, port_number: u32) -> Result<NetscriptPort, BitburnerError> {
        self.get_port_handle_shim(port_number)
    }

    /// Check if a script is running.
    ///
    /// **RAM cost: 0.1 GB**
    ///
    /// Returns a boolean indicating whether the specified script is running on
    /// the target server. If you use a PID instead of a filename, the
    /// hostname and args parameters are unnecessary. If hostname is omitted
    /// while filename is used as the first parameter, hostname defaults to the
    /// server the calling script is running on. Remember that a script is
    /// semi-uniquely identified by both its name and its arguments.
    /// (You can run multiple copies of scripts with the same arguments, but for
    /// the purposes of functions like this that check based on filename,
    /// the filename plus arguments forms the key.)
    ///
    /// # Arguments
    /// * script - Filename or PID of script to check. This is case-sensitive.
    /// * host - Hostname of target server. Optional, defaults to the server the
    ///   calling script is running on.
    /// * args - Arguments to specify/identify the script. Optional, when
    ///   looking for scripts run without arguments.
    pub fn is_running(
        self: &NS,
        script: FilenameOrPID,
        host: Option<&str>,
        args: Vec<Arg>,
    ) -> Result<bool, BitburnerError> {
        self.is_running_shim(&script.into(), host, &args.to_js())
    }

    /// Get general info about a running script.
    ///
    /// **RAM cost: 0.3 GB**
    ///
    /// Running with no args returns current script.
    /// If you use a PID as the first parameter, the hostname and args
    /// parameters are unnecessary. If hostname is omitted while filename is
    /// used as the first parameter, hostname defaults to the server the calling
    /// script is running on. Remember that a script is semi-uniquely
    /// identified by both its name and its arguments. (You can run multiple
    /// copies of scripts with the same arguments, but for the purposes of
    /// functions like this that check based on filename, the filename plus
    /// arguments forms the key.)
    ///
    /// Returns the info about the running script if found, and [`None`]
    /// otherwise.
    ///
    /// # Arguments
    /// * filename - Optional. Filename or PID of the script.
    /// * hostname - Hostname of target server. Optional, defaults to the server
    ///   the calling script is running on.
    /// * args  - Arguments to specify/identify the script. Optional, when
    ///   looking for scripts run without arguments.
    pub fn get_running_script(
        self: &NS,
        filename: Option<FilenameOrPID>,
        hostname: Option<&str>,
        args: Vec<Arg>,
    ) -> Result<Option<RunningScript>, BitburnerError> {
        let filename = filename.into();
        let hostname = hostname.map(|s| s.to_owned());
        self.get_running_script_shim(&filename, hostname.as_deref(), &args.to_js())
    }

    /// Get the execution time of a [`NS::hack`] call.
    ///
    /// **RAM cost: 0.05 GB**
    ///
    /// When `hack` completes an amount of money is stolen depending on the
    /// player's skills. Returns the amount of time in milliseconds it takes
    /// to execute the [`NS::hack`] Netscript function on the target server.
    /// The required time is increased by the security level of the target
    /// server and decreased by the player's hacking level.
    ///
    /// Returns the amount of time in milliseconds it takes to execute the
    /// [`NS::hack`] Netscript function.
    pub fn get_hack_time(self: &NS, host: &str) -> Result<f64, BitburnerError> {
        self.get_hack_time_shim(host)
    }

    /// Get the execution time of a [`NS::grow`] call.
    ///
    /// **RAM cost: 0.05 GB**
    ///
    /// Returns the amount of time in milliseconds it takes to execute the
    /// [`NS::grow`] Netscript function on the target server. The required
    /// time is increased by the security level of the target server and
    /// decreased by the player's hacking level.
    ///
    /// Returns the amount of time in milliseconds it takes to execute the
    /// [`NS::grow`] Netscript function.
    pub fn get_grow_time(self: &NS, host: &str) -> Result<f64, BitburnerError> {
        self.get_grow_time_shim(host)
    }

    /// Get the execution time of a [`NS::weaken`] call.
    ///
    /// **RAM cost: 0.05 GB**
    ///
    /// Returns the amount of time in milliseconds it takes to execute the
    /// [`NS::weaken`] Netscript function on the target server. The required
    /// time is increased by the security level of the target server and
    /// decreased by the player's hacking level.
    ///
    /// Returns the amount of time in milliseconds it takes to execute the
    /// [`NS::weaken`] Netscript function.
    pub fn get_weaken_time(self: &NS, host: &str) -> Result<f64, BitburnerError> {
        self.get_weaken_time_shim(host)
    }
}
