use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OldVse {
    pub workload: Vec<Workload>,
    pub settings: Settings,
    pub sites: Vec<Site>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Workload {
    pub workload_active: String,
    pub work_load_name: String,
    pub backup_type: String,
    pub site: String,
    pub work_load_cap: f64,
    pub growth_percent: i64,
    pub scope_years: i64,
    pub backup_window: i64,
    pub change_rate: i64,
    pub reduction: i64,
    pub vm_qty: i64,
    pub vm_vmdk_ratio: i64,
    #[serde(rename = "usePerVM")]
    pub use_per_vm: String,
    pub use_re_fs: String,
    pub rps_bu: i64,
    pub bu_weekly: i64,
    pub bu_monthly: i64,
    pub bu_yearly: i64,
    pub cloud_move: i64,
    pub copy_site: String,
    pub rps_bu_copy: i64,
    pub bu_copy_weekly: i64,
    pub bu_copy_monthly: i64,
    pub bu_copy_yearly: i64,
    pub cloud_enabled: bool,
    pub process_capacity: f64,
    pub band_width_inc: f64,
    pub vmdk_qty: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    pub server_min: ServerMin,
    pub vbr_settings: VbrSettings,
    pub proxy_settings: ProxySettings,
    pub repo_settings: RepoSettings,
    pub em_settings: EmSettings,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerMin {
    pub vbr_server: VbrServer,
    pub sql_server: SqlServer,
    pub v_proxy_server: VProxyServer,
    pub repo_server: RepoServer,
    pub em_server: EmServer,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VbrServer {
    pub cores: i64,
    pub ram: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SqlServer {
    pub cores: i64,
    pub ram: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VProxyServer {
    pub cores: i64,
    pub ram: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RepoServer {
    pub cores: i64,
    pub ram: i64,
    pub capacity: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmServer {
    pub cores: i64,
    pub ram: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VbrSettings {
    #[serde(rename = "numVMwithPerVM")]
    pub num_vmwith_per_vm: i64,
    pub num_vm_withper_job: i64,
    pub vbr_concurrent_jobs: i64,
    pub con_jobs_for_cores: i64,
    pub con_jobs_for_mem: i64,
    #[serde(rename = "coresFor25ConJobs")]
    pub cores_for25con_jobs: i64,
    #[serde(rename = "memFor25ConJobs")]
    pub mem_for25con_jobs: i64,
    pub mem_per_con_jobs: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProxySettings {
    pub ingest_per_cpu_core_full: i64,
    pub ingest_per_cpu_core_inc: i64,
    pub proxy_task_consumes_mem: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RepoSettings {
    pub daily_crm: i64,
    pub weekly_crm: i64,
    pub monthly_crm: i64,
    pub yearly_crm: i64,
    pub repo_task_con_memory: i64,
    pub task_core_ratio: i64,
    pub use_rpc: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EmSettings {
    pub em_use_api_mem_add: i64,
    pub em_use_api_core_add: i64,
    pub em_use_multi_vbr_mem_add: i64,
    pub em_use_multi_vbr_cores_add: i64,
    pub em_use_self_mem_add: i64,
    pub em_use_self_cores_add: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Site {
    pub site_name: String,
    pub wan_speed: i64,
    pub internet_speed: i64,
    pub network_speed: i64,
}
