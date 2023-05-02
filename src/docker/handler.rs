const DAEMONS: [&str; 1] = ["127.0.0.1:2375"];

#[derive(Clone)]
pub struct State {
    pub clients: Vec<super::client::Client>,
}

/*
   [REQ]
   HEAD /_ping HTTP/1.1
   Host: 127.0.0.1:2377
   User-Agent: Docker-Client/23.0.5 (windows)

   [RES]
   HTTP/1.1 200 OK
   Api-Version: 1.42
   Builder-Version: 2
   Cache-Control: no-cache, no-store, must-revalidate
   Content-Type: text/plain; charset=utf-8
   Date: Tue, 02 May 2023 20:53:55 GMT
   Docker-Experimental: false
   Ostype: linux
   Pragma: no-cache
   Server: Docker/23.0.5 (linux)
   Swarm: inactive
*/

pub async fn ping() -> impl axum::response::IntoResponse {
    // let mut headers = axum::http::HeaderMap::new();
    // *headers.get_mut("Api-Version").unwrap() = "1.42".parse().unwrap();

    (
        [
            ("Cache-Control", "no-cache, no-store, must-revalidate"),
            ("Content-Type", "text/plain; charset=utf-8"),
            ("Api-Version", "1.42"),
            ("Builder-Version", "2"),
            ("Docker-Experimental", "false"),
            ("Ostype", "linux"),
            ("Pragma", "no-cache"),
            ("Server", "Docker/23.0.5 (linux)"),
            ("Swarm", "inactive"),
        ],
        "OK",
    )
}

/*
   [REQ]
   GET /v1.42/info HTTP/1.1
   Host: 127.0.0.1:2377
   User-Agent: Docker-Client/23.0.5 (windows)

   [RES]
   HTTP/1.1 200 OK
   Api-Version: 1.42
   Content-Type: application/json
   Date: Tue, 02 May 2023 20:53:55 GMT
   Docker-Experimental: false
   Ostype: linux
   Server: Docker/23.0.5 (linux)
   Transfer-Encoding: chunked

   b80
   {\"ID\":\"bb2f7c74-92f1-4ada-a7a0-040228185348\",\"Containers\":3,\"ContainersRunning\":2,\"ContainersPaused\":0,\"ContainersStopped\":1,\"Images\":14,\"Driver\":\"overlay2\",\"DriverStatus\":[[\"Backing Filesystem\",\"extfs\"],[\"Supports d_type\",\"true\"],[\"Using metacopy\",\"false\"],[\"Native Overlay Diff\",\"true\"],[\"userxattr\",\"false\"]],\"Plugins\":{\"Volume\":[\"local\"],\"Network\":[\"bridge\",\"host\",\"ipvlan\",\"macvlan\",\"null\",\"overlay\"],\"Authorization\":null,\"Log\":[\"awslogs\",\"fluentd\",\"gcplogs\",\"gelf\",\"journald\",\"json-file\",\"local\",\"logentries\",\"splunk\",\"syslog\"]},\"MemoryLimit\":true,\"SwapLimit\":true,\"KernelMemoryTCP\":true,\"CpuCfsPeriod\":true,\"CpuCfsQuota\":true,\"CPUShares\":true,\"CPUSet\":true,\"PidsLimit\":true,\"IPv4Forwarding\":true,\"BridgeNfIptables\":true,\"BridgeNfIp6tables\":true,\"Debug\":false,\"NFd\":76,\"OomKillDisable\":true,\"NGoroutines\":104,\"SystemTime\":\"2023-05-02T20:53:55.086098154Z\",\"LoggingDriver\":\"json-file\",\"CgroupDriver\":\"cgroupfs\",\"CgroupVersion\":\"1\",\"NEventsListener\":9,\"KernelVersion\":\"5.15.90.1-microsoft-standard-WSL2\",\"OperatingSystem\":\"Docker Desktop\",\"OSVersion\":\"\",\"OSType\":\"linux\",\"Architecture\":\"x86_64\",\"IndexServerAddress\":\"https://index.docker.io/v1/\",\"RegistryConfig\":{\"AllowNondistributableArtifactsCIDRs\":null,\"AllowNondistributableArtifactsHostnames\":null,\"InsecureRegistryCIDRs\":[\"127.0.0.0/8\"],\"IndexConfigs\":{\"docker.io\":{\"Name\":\"docker.io\",\"Mirrors\":[],\"Secure\":true,\"Official\":true},\"hubproxy.docker.internal:5555\":{\"Name\":\"hubproxy.docker.internal:5555\",\"Mirrors\":[],\"Secure\":false,\"Official\":false}},\"Mirrors\":null},\"NCPU\":12,\"MemTotal\":20932829184,\"GenericResources\":null,\"DockerRootDir\":\"/var/lib/docker\",\"HttpProxy\":\"http.docker.internal:3128\",\"HttpsProxy\":\"http.docker.internal:3128\",\"NoProxy\":\"hubproxy.docker.internal\",\"Name\":\"docker-desktop\",\"Labels\":[],\"ExperimentalBuild\":false,\"ServerVersion\":\"23.0.5\",\"Runtimes\":{\"io.containerd.runc.v2\":{\"path\":\"runc\"},\"runc\":{\"path\":\"runc\"}},\"DefaultRuntime\":\"runc\",\"Swarm\":{\"NodeID\":\"\",\"NodeAddr\":\"\",\"LocalNodeState\":\"inactive\",\"ControlAvailable\":false,\"Error\":\"\",\"RemoteManagers\":null},\"LiveRestoreEnabled\":false,\"Isolation\":\"\",\"InitBinary\":\"docker-init\",\"ContainerdCommit\":{\"ID\":\"2806fc1057397dbaeefbea0e4e17bddfbd388f38\",\"Expected\":\"2806fc1057397dbaeefbea0e4e17bddfbd388f38\"},\"RuncCommit\":{\"ID\":\"v1.1.5-0-gf19387a\",\"Expected\":\"v1.1.5-0-gf19387a\"},\"InitCommit\":{\"ID\":\"de40ad0\",\"Expected\":\"de40ad0\"},\"SecurityOptions\":[\"name=seccomp,profile=builtin\"],\"Warnings\":[\"WARNING: API is accessible on http://0.0.0.0:2376 without encryption.\\n         Access to the remote API is equivalent to root access on the host. Refer\\n         to the 'Docker daemon attack surface' section in the documentation for\\n         more information: https://docs.docker.com/go/attack-surface/\",\"WARNING: No blkio throttle.read_bps_device support\",\"WARNING: No blkio throttle.write_bps_device support\",\"WARNING: No blkio throttle.read_iops_device support\",\"WARNING: No blkio thro
   0
*/

pub async fn info(
    axum::extract::State(state): axum::extract::State<State>,
) -> impl axum::response::IntoResponse {
    let res =
        futures::future::join_all(state.clients.iter().map(|c| c.info()).collect::<Vec<_>>()).await;

    let info = res
        .iter()
        .filter(|i| i.is_ok())
        .map(|i| i.as_ref().unwrap())
        .collect::<Vec<_>>();

    let info = super::dtos::EngineInfo {
        id: "77777777-7777-7777-7777-777777777777".to_string(),
        containers: info.iter().map(|i| i.containers).sum(),
        containers_running: info.iter().map(|i| i.containers_running).sum(),
        containers_paused: info.iter().map(|i| i.containers_paused).sum(),
        containers_stopped: info.iter().map(|i| i.containers_stopped).sum(),
        images: info.iter().map(|i| i.images).sum(),
        driver: "overlay2".to_string(),
        driver_status: vec![
            ["Backing Filesystem".to_string(), "extfs".to_string()],
            ["Supports d_type".to_string(), "true".to_string()],
            ["Using metacopy".to_string(), "false".to_string()],
            ["Native Overlay Diff".to_string(), "true".to_string()],
            ["userxattr".to_string(), "false".to_string()],
        ],
        plugins: [
            ("Volume".to_string(), Some(vec!["local".to_string()])),
            (
                "Network".to_string(),
                Some(vec![
                    "bridge".to_string(),
                    "host".to_string(),
                    "ipvlan".to_string(),
                    "macvlan".to_string(),
                    "null".to_string(),
                    "overlay".to_string(),
                ]),
            ),
            ("Authorization".to_string(), None),
            (
                "Log".to_string(),
                Some(vec![
                    "awslogs".to_string(),
                    "fluentd".to_string(),
                    "gcplogs".to_string(),
                    "gelf".to_string(),
                    "journald".to_string(),
                    "json-file".to_string(),
                    "local".to_string(),
                    "logentries".to_string(),
                    "splunk".to_string(),
                    "syslog".to_string(),
                ]),
            ),
        ]
        .into(),
        memory_limit: true,
        swap_limit: true,
        kernel_memory_tcp: true,
        cpu_cfs_period: true,
        cpu_cfs_quota: true,
        cpu_shares: true,
        cpu_set: true,
        pids_limit: true,
        ipv4_forwarding: true,
        bridge_nf_iptables: true,
        bridge_nf_ip6tables: true,
        debug: false,
        n_fd: info.iter().map(|i| i.n_fd).sum(),
        oom_kill_disable: true,
        n_goroutines: info.iter().map(|i| i.n_goroutines).sum(),
        system_time: chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
        logging_driver: "json-file".to_string(),
        cgroup_driver: "cgroupfs".to_string(),
        n_events_listener: info.iter().map(|i| i.n_events_listener).sum(),
        // TODO: fetch from a random daemon
        kernel_version: "fake-kernel".to_string(),
        // TODO: fetch from a random daemon
        operating_system: "fake-os".to_string(),
        // TODO: fetch from a random daemon
        os_version: "fake-os-version".to_string(),
        // TODO: fetch from a random daemon
        os_type: "linux".to_string(),
        // TODO: fetch from a random daemon
        architecture: "x86_64".to_string(),
        // TODO: fetch from a random daemon
        index_server_address: "https://index.docker.io/v1/".to_string(),
        registry_config: super::dtos::RegistryConfig {
            allow_nondistributable_artifacts_cidrs: None,
            allow_nondistributable_artifacts_hostnames: None,
            insecure_registry_cidrs: Some(vec!["127.0.0.0/8".to_string()]),
            index_configs: [
                (
                    "docker.io".to_string(),
                    super::dtos::IndexConfig {
                        name: "docker.io".to_string(),
                        mirrors: None,
                        secure: true,
                        official: true,
                    },
                ),
                (
                    "hubproxy.docker.internal:5555".to_string(),
                    super::dtos::IndexConfig {
                        name: "hubproxy.docker.internal:5555".to_string(),
                        mirrors: None,
                        secure: false,
                        official: false,
                    },
                ),
            ]
            .into(),
            mirrors: None,
        },
        n_cpu: info.iter().map(|i| i.n_cpu).sum(),
        mem_total: info.iter().map(|i| i.mem_total).sum(),
        generic_resources: None,
        docker_root_dir: "/var/lib/docker".to_string(),
        http_proxy: "http://docker.internal:3128".to_string(),
        https_proxy: "http://docker.internal:3128".to_string(),
        no_proxy: "hubproxy.docker.internal".to_string(),
        name: "multidocker".to_string(),
        labels: vec![],
        experimental_build: false,
        // TODO: fetch from random daemon
        server_version: "23.0.5".to_string(),
        // TODO: fetch from all daemons
        runtimes: [
            (
                "io.containerd.runc.v2".to_string(),
                super::dtos::Runtime {
                    path: "runc".to_string(),
                },
            ),
            (
                "runc".to_string(),
                super::dtos::Runtime {
                    path: "runc".to_string(),
                },
            ),
        ]
        .into(),
        default_runtime: "runc".to_string(),
        swarm: super::dtos::Swarm {
            node_id: "".to_string(),
            node_addr: "".to_string(),
            local_node_state: super::dtos::SwarmNodeState::Inactive,
            control_available: false,
            error: "".to_string(),
            remote_managers: None,
        },
        live_restore_enabled: false,
        isolation: "".to_string(),
        init_binary: "docker-init".to_string(),
        containerd_commit: super::dtos::Commit {
            id: "0000000".to_string(),
            expected: "0000000".to_string(),
        },
        runc_commit: super::dtos::Commit {
            id: "0000000".to_string(),
            expected: "0000000".to_string(),
        },
        init_commit: super::dtos::Commit {
            id: "0000000".to_string(),
            expected: "0000000".to_string(),
        },
        security_options: vec!["name=seccomp,profile=builtin".to_string()],
        warnings: vec![format!(
            "Fetched info from {} daemon(s)",
            state.clients.len()
        )],
    };

    (
        [
            ("Cache-Control", "no-cache, no-store, must-revalidate"),
            ("Content-Type", "text/plain; charset=utf-8"),
            ("Api-Version", "1.42"),
            ("Builder-Version", "2"),
            ("Docker-Experimental", "false"),
            ("Ostype", "linux"),
            ("Pragma", "no-cache"),
            ("Server", "Docker/23.0.5 (linux)"),
            ("Swarm", "inactive"),
        ],
        axum::Json(info),
    )
}
