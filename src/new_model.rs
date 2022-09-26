use serde::Deserialize;
use serde::Serialize;


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewVse {
    pub project_length: i64,
    pub show_points: bool,
    pub show_workloads: bool,
    pub sites: Vec<Site>,
    pub perf_tier_repos: Vec<PerfTierRepo>,
    pub cap_tier_repos: Vec<CapTierRepo>,
    pub arch_tier_repos: Vec<ArchTierRepo>,
    pub data_properties: Vec<DataProperty>,
    pub backup_windows: Vec<BackupWindow>,
    pub retentions: Vec<Retentions>,
    pub workloads: Vec<Workload>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Site {
    pub site_id: String,
    pub site_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PerfTierRepo {
    pub repo_id: String,
    pub repo_name: String,
    pub site_id: String,
    pub copy_capacity_tier_enabled: bool,
    pub move_capacity_tier_enabled: bool,
    pub archive_tier_enabled: bool,
    pub capacity_tier_days: i64,
    pub archive_tier_days: i64,
    pub capacity_tier_repo_id: String,
    pub archive_tier_repo_id: String,
    #[serde(rename = "usePerVM")]
    pub use_per_vm: bool,
    pub block_clone_enabled: bool,
    pub object_storage: bool,
    pub immutable_perf: bool,
    pub immutable_cap: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CapTierRepo {
    pub cap_tier_repo_id: String,
    pub cap_tier_repo_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchTierRepo {
    pub archive_tier_repo_id: String,
    pub archive_tier_repo_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataProperty {
    pub data_property_id: String,
    pub data_property_name: String,
    pub change_rate: i64,
    pub compression: i64,
    pub growth_factor: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackupWindow {
    pub backup_window_id: String,
    pub backup_window_name: String,
    pub full_window: i64,
    pub incremental_window: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Retentions {
    pub retention_id: String,
    pub retention_name: String,
    pub simple: i64,
    pub weekly: i64,
    pub monthly: i64,
    pub yearly: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Workload {
    pub workload_id: String,
    pub enabled: bool,
    pub workload_name: String,
    pub site_id: String,
    pub large_blocks: bool,
    #[serde(rename = "sourceTB")]
    pub source_tb: f64,
    pub units: i64,
    pub workload_type: String,
    pub data_property_id: String,
    pub backup: Backup,
    pub copies_enabled: bool,
    pub copies: Option<Vec<Copy>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Backup {
    pub retention_id: String,
    pub repo_id: String,
    pub backup_window_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Copy {
    pub retention_id: String,
    pub repo_id: String,
    pub backup_window_id: String,
}
