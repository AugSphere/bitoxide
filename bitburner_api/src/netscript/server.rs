use wasm_bindgen::prelude::*;

impl Server {
    /// Hostname. Must be unique.
    pub fn hostname(&self) -> String {
        self.hostname_shim()
    }

    /// IP Address. Must be unique.
    pub fn ip(&self) -> String {
        self.ip_shim()
    }

    /// Whether or not the SSH Port is open.
    pub fn ssh_port_open(&self) -> bool {
        self.ssh_port_open_shim()
    }

    /// Whether or not the FTP port is open.
    pub fn ftp_port_open(&self) -> bool {
        self.ftp_port_open_shim()
    }

    /// Whether or not the SMTP Port is open.
    pub fn smtp_port_open(&self) -> bool {
        self.smtp_port_open_shim()
    }

    /// Whether or not the HTTP Port is open.
    pub fn http_port_open(&self) -> bool {
        self.http_port_open_shim()
    }

    /// Whether or not the SQL Port is open.
    pub fn sql_port_open(&self) -> bool {
        self.sql_port_open_shim()
    }

    /// Flag indicating whether player has admin/root access to this server.
    pub fn has_admin_rights(&self) -> bool {
        self.has_admin_rights_shim()
    }

    /// How many CPU cores this server has.
    ///
    /// Affects magnitude of grow and weaken ran from this server.
    pub fn cpu_coes(&self) -> u32 {
        self.cpu_cores_shim()
    }

    /// Flag indicating whether player is currently connected to this server.
    pub fn is_connected_to(&self) -> bool {
        self.is_connected_to_shim()
    }

    /// RAM (GB) used. i.e. unavailable RAM.
    pub fn ram_used(&self) -> f64 {
        self.ram_used_shim()
    }

    /// RAM (GB) available on this server.
    pub fn max_ram(&self) -> f64 {
        self.max_ram_shim()
    }

    /// Name of company/faction/etc. that this server belongs to, not applicable
    /// to all Servers.
    pub fn organization_name(&self) -> String {
        self.organization_name_shim()
    }

    /// Flag indicating whether this is a purchased server.
    pub fn purchased_by_player(&self) -> bool {
        self.purchased_by_player_shim()
    }

    /// Flag indicating whether this server has a backdoor installed by a
    /// player.
    pub fn backdoor_installed(&self) -> bool {
        self.backdoor_installed_shim()
    }

    /// Server's initial server security level at creation.
    pub fn base_difficulty(&self) -> f64 {
        self.base_difficulty_shim()
    }

    /// Server Security Level.
    pub fn hack_difficulty(&self) -> f64 {
        self.hack_difficulty_shim()
    }

    /// Minimum server security level that this server can be weakened to.
    pub fn min_difficulty(&self) -> f64 {
        self.min_difficulty_shim()
    }

    /// How much money currently resides on the server and can be hacked.
    pub fn money_available(&self) -> f64 {
        self.money_available_shim()
    }

    /// Maximum amount of money that this server can hold.
    pub fn money_max(&self) -> f64 {
        self.money_max_shim()
    }

    /// Number of open ports required in order to gain admin/root access.
    pub fn num_open_ports_required(&self) -> u32 {
        self.num_open_ports_required_shim()
    }

    /// How many ports are currently opened on the server.
    pub fn open_port_count(&self) -> u32 {
        self.open_port_count_shim()
    }

    /// Hacking level required to hack this server.
    pub fn required_hacking_skill(&self) -> f64 {
        self.required_hacking_skill_shim()
    }

    /// Growth effectiveness statistic.
    ///
    /// Higher values produce more growth with [`grow`](crate::NS::grow).
    pub fn server_growth(&self) -> f64 {
        self.server_growth_shim()
    }
}

#[wasm_bindgen]
extern "C" {
    pub type Server;

    #[wasm_bindgen(method, getter, js_name = hostname)]
    fn hostname_shim(this: &Server) -> String;

    #[wasm_bindgen(method, getter, js_name = ip)]
    fn ip_shim(this: &Server) -> String;

    #[wasm_bindgen(method, getter, js_name = sshPortOpen)]
    fn ssh_port_open_shim(this: &Server) -> bool;

    #[wasm_bindgen(method, getter, js_name = ftpPortOpen)]
    fn ftp_port_open_shim(this: &Server) -> bool;

    #[wasm_bindgen(method, getter, js_name = smtpPortOpen)]
    fn smtp_port_open_shim(this: &Server) -> bool;

    #[wasm_bindgen(method, getter, js_name = httpPortOpen)]
    fn http_port_open_shim(this: &Server) -> bool;

    #[wasm_bindgen(method, getter, js_name = sqlPortOpen)]
    fn sql_port_open_shim(this: &Server) -> bool;

    #[wasm_bindgen(method, getter, js_name = hasAdminRights)]
    fn has_admin_rights_shim(this: &Server) -> bool;

    #[wasm_bindgen(method, getter, js_name = cpuCores)]
    fn cpu_cores_shim(this: &Server) -> u32;

    #[wasm_bindgen(method, getter, js_name = isConnectedTo)]
    fn is_connected_to_shim(this: &Server) -> bool;

    #[wasm_bindgen(method, getter, js_name = ramUsed)]
    fn ram_used_shim(this: &Server) -> f64;

    #[wasm_bindgen(method, getter, js_name = maxRam)]
    fn max_ram_shim(this: &Server) -> f64;

    #[wasm_bindgen(method, getter, js_name = organizationName)]
    fn organization_name_shim(this: &Server) -> String;

    #[wasm_bindgen(method, getter, js_name = purchasedByPlayer)]
    fn purchased_by_player_shim(this: &Server) -> bool;

    #[wasm_bindgen(method, getter, js_name = backdoorInstalled)]
    fn backdoor_installed_shim(this: &Server) -> bool;

    #[wasm_bindgen(method, getter, js_name = baseDifficulty)]
    fn base_difficulty_shim(this: &Server) -> f64;

    #[wasm_bindgen(method, getter, js_name = hackDifficulty)]
    fn hack_difficulty_shim(this: &Server) -> f64;

    #[wasm_bindgen(method, getter, js_name = minDifficulty)]
    fn min_difficulty_shim(this: &Server) -> f64;

    #[wasm_bindgen(method, getter, js_name = moneyAvailable)]
    fn money_available_shim(this: &Server) -> f64;

    #[wasm_bindgen(method, getter, js_name = moneyMax)]
    fn money_max_shim(this: &Server) -> f64;

    #[wasm_bindgen(method, getter, js_name = numOpenPortsRequired)]
    fn num_open_ports_required_shim(this: &Server) -> u32;

    #[wasm_bindgen(method, getter, js_name = openPortCount)]
    fn open_port_count_shim(this: &Server) -> u32;

    #[wasm_bindgen(method, getter, js_name = requiredHackingSkill)]
    fn required_hacking_skill_shim(this: &Server) -> f64;

    #[wasm_bindgen(method, getter, js_name = serverGrowth)]
    fn server_growth_shim(this: &Server) -> f64;
}
