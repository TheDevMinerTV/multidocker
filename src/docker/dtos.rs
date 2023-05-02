#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EngineInfo {
    #[serde(rename = "ID")]
    pub id: String,
    pub containers: u32,
    pub containers_running: u32,
    pub containers_paused: u32,
    pub containers_stopped: u32,
    pub images: u32,
    pub driver: String,
    pub driver_status: Vec<[String; 2]>,
    pub plugins: std::collections::HashMap<String, Option<Vec<String>>>,
    pub memory_limit: bool,
    pub swap_limit: bool,
    #[serde(rename = "KernelMemoryTCP")]
    pub kernel_memory_tcp: bool,
    pub cpu_cfs_period: bool,
    pub cpu_cfs_quota: bool,
    #[serde(rename = "CPUShares")]
    pub cpu_shares: bool,
    #[serde(rename = "CPUSet")]
    pub cpu_set: bool,
    pub pids_limit: bool,
    #[serde(rename = "IPv4Forwarding")]
    pub ipv4_forwarding: bool,
    pub bridge_nf_iptables: bool,
    pub bridge_nf_ip6tables: bool,
    pub debug: bool,
    pub n_fd: u32,
    pub oom_kill_disable: bool,
    pub n_goroutines: u32,
    pub system_time: String,
    pub logging_driver: String,
    pub cgroup_driver: String,
    pub n_events_listener: u32,
    pub kernel_version: String,
    pub operating_system: String,
    #[serde(rename = "OSVersion")]
    pub os_version: String,
    #[serde(rename = "OSType")]
    pub os_type: String,
    pub architecture: String,
    pub index_server_address: String,
    pub registry_config: RegistryConfig,
    #[serde(rename = "NCPU")]
    pub n_cpu: u32,
    pub mem_total: u64,
    pub generic_resources: Option<Vec<serde_json::Value>>,
    pub docker_root_dir: String,
    pub http_proxy: String,
    pub https_proxy: String,
    pub no_proxy: String,
    pub name: String,
    pub labels: Vec<String>,
    pub experimental_build: bool,
    pub server_version: String,
    pub runtimes: std::collections::HashMap<String, Runtime>,
    pub default_runtime: String,
    pub swarm: Swarm,
    pub live_restore_enabled: bool,
    pub isolation: String,
    pub init_binary: String,
    pub containerd_commit: Commit,
    pub runc_commit: Commit,
    pub init_commit: Commit,
    pub security_options: Vec<String>,
    pub warnings: Vec<String>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RegistryConfig {
    #[serde(rename = "AllowNondistributableArtifactsCIDRs")]
    pub allow_nondistributable_artifacts_cidrs: Option<Vec<String>>,
    pub allow_nondistributable_artifacts_hostnames: Option<Vec<String>>,
    #[serde(rename = "InsecureRegistryCIDRs")]
    pub insecure_registry_cidrs: Option<Vec<String>>,
    pub index_configs: std::collections::HashMap<String, IndexConfig>,
    pub mirrors: Option<Vec<String>>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct IndexConfig {
    pub name: String,
    pub mirrors: Option<Vec<String>>,
    pub secure: bool,
    pub official: bool,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Runtime {
    #[serde(rename = "path")]
    pub path: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Swarm {
    #[serde(rename = "NodeID")]
    pub node_id: String,
    pub node_addr: String,
    pub local_node_state: SwarmNodeState,
    pub control_available: bool,
    pub error: String,
    pub remote_managers: Option<Vec<serde_json::Value>>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SwarmNodeState {
    Inactive,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Commit {
    #[serde(rename = "ID")]
    pub id: String,
    pub expected: String,
}
