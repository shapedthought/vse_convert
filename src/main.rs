mod new_model;
mod vse_model;

use anyhow::Result;
use colored::*;
use new_model::Backup as NewBackup;
use new_model::Copy as NewCopy;
use new_model::Site as NewSite;
use new_model::{BackupWindow, DataProperty, PerfTierRepo, RetentionPolicy, Workload};
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use vse_model::OldVse;

use crate::new_model::{ArchTierRepo, CapTierRepo, NewVse};

use clap::Parser;
use dialoguer::Input;

#[derive(Parser)]
#[clap(author, version, about)]
#[clap(propagate_version = true)]
struct Cli {
    /// VSE workload file
    #[clap(short, long, value_parser)]
    vse_file: PathBuf,

    /// The new file name, without extension
    #[clap(short, long, value_parser)]
    save_file: String,

    /// Pring the result
    #[clap(short, long, action, default_value_t = false)]
    print: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let file = fs::read_to_string(cli.vse_file)?;
    let old_vse: OldVse = serde_json::from_str(&file)?;

    let workloads = old_vse.workload;

    let sites = old_vse.sites;

    let mut sites_hash = HashMap::new();

    let project_length = workloads.iter().map(|x| x.scope_years).max().unwrap();

    for site in sites {
        sites_hash.insert(site.site_name.to_lowercase(), site.site_name.to_lowercase());
    }

    let mut data_properties: HashMap<String, DataProperty> = HashMap::new();
    let mut backup_window: HashMap<String, BackupWindow> = HashMap::new();
    let mut retentions: HashMap<String, RetentionPolicy> = HashMap::new();
    let mut perf_repos: HashMap<String, PerfTierRepo> = HashMap::new();
    let mut new_workloads: Vec<Workload> = Vec::new();

    let mut wl_id: u32 = 0;

    for item in workloads.iter() {
        let dp_id_name = format!(
            "CR{}C{}G{}",
            item.change_rate,
            100 - item.reduction,
            item.growth_percent
        );

        let dp = DataProperty {
            data_property_id: dp_id_name.clone(),
            data_property_name: dp_id_name.clone(),
            change_rate: item.change_rate,
            compression: 100 - item.reduction,
            growth_factor: item.growth_percent,
        };
        data_properties.insert(dp_id_name.clone(), dp);

        let bw_id_name = format!("F24I{}", item.backup_window);

        let bw = BackupWindow {
            backup_window_id: bw_id_name.clone(),
            backup_window_name: bw_id_name.clone(),
            full_window: 24,
            incremental_window: item.backup_window,
        };
        backup_window.insert(bw_id_name.clone(), bw);

        let rt_id_name = format!(
            "{}D{}W{}M{}Y",
            item.rps_bu, item.bu_weekly, item.bu_monthly, item.bu_yearly
        );

        let rp = RetentionPolicy {
            retention_policy_id: rt_id_name.clone(),
            retention_policy_name: rt_id_name.clone(),
            simple: item.rps_bu,
            weekly: item.bu_weekly,
            monthly: item.bu_monthly,
            yearly: item.bu_yearly,
        };

        retentions.insert(rt_id_name.clone(), rp);
        

        let perf_id_name = format!("repo_{}", item.site.to_lowercase());
        let cap_tier_copy: bool;

        if !perf_repos.contains_key(&perf_id_name) {
            let dia_text = format!("Enable Capacity Tier Copy on {perf_id_name}?");
            cap_tier_copy = Input::<bool>::new()
                .with_prompt(dia_text)
                .default(false)
                .interact_text()?;
        } else {
            cap_tier_copy = perf_repos.get(&perf_id_name).unwrap().copy_capacity_tier_enabled;
        }

        let perf_repo = PerfTierRepo {
            repo_id: perf_id_name.clone(),
            repo_name: perf_id_name.clone(),
            site_id: item.site.to_lowercase(),
            copy_capacity_tier_enabled: cap_tier_copy,
            move_capacity_tier_enabled: item.cloud_enabled,
            archive_tier_enabled: false,
            capacity_tier_days: item.cloud_move,
            capacity_tier_repo_id: "ct1".to_string(),
            archive_tier_repo_id: "at1".to_string(),
            archive_tier_days: 0,
            use_per_vm: if item.use_per_vm == "no" { false } else { true },
            block_clone_enabled: if item.use_re_fs == "no" { false } else { true },
            object_storage: false,
            immutable_cap: false,
            immutable_perf: false,
        };
        // perf_repos.push(perf_repo);
        perf_repos.insert(perf_id_name.clone(), perf_repo);

        let mut rps_copies: Option<Vec<NewCopy>> = None;

        if item.copy_site != "None" {
            let copy = NewCopy {
                retention_policy_id: format!(
                    "{}D{}W{}M{}Y",
                    item.rps_bu_copy,
                    item.bu_copy_weekly,
                    item.bu_copy_monthly,
                    item.bu_copy_yearly
                ),
                repo_id: format!("repo_{}", item.copy_site.to_lowercase()),
                backup_window_id: bw_id_name.clone(),
            };
            let copies = vec![copy];
            rps_copies = Some(copies)
        }

        let new_workload = Workload {
            workload_id: format!("wl{}", wl_id),
            enabled: true,
            workload_name: item.work_load_name.clone(),
            site_id: item.site.to_lowercase(),
            large_blocks: false,
            source_tb: item.work_load_cap,
            units: item.vm_qty,
            workload_type: item.backup_type.to_uppercase(),
            data_property_id: dp_id_name,
            backup: NewBackup {
                retention_policy_id: rt_id_name,
                repo_id: perf_id_name,
                backup_window_id: bw_id_name,
            },
            copies_enabled: if item.copy_site != "None" {
                true
            } else {
                false
            },
            copies: rps_copies,
        };
        new_workloads.push(new_workload);

        wl_id += 1;
    }

    let new_sites: Vec<NewSite> = sites_hash
        .iter()
        .map(|w| NewSite {
            site_id: w.0.to_string(),
            site_name: w.1.to_string(),
        })
        .collect();

    let cap_repos = vec![CapTierRepo {
        cap_tier_repo_id: "ct1".to_string(),
        cap_tier_repo_name: "ct1".to_string(),
    }];

    let arch_repos = vec![ArchTierRepo {
        archive_tier_repo_id: "at1".to_string(),
        archive_tier_repo_name: "at1".to_string(),
    }];

    let retentions_vec = retentions.into_values().collect();
    let backupwindow_vec = backup_window.into_values().collect();
    let dataproperty_vec = data_properties.into_values().collect();
    let perf_vec = perf_repos.into_values().collect();

    let new_vse = NewVse {
        project_length: project_length,
        show_points: false,
        sites: new_sites,
        perf_tier_repos: perf_vec,
        cap_tier_repos: cap_repos,
        arch_tier_repos: arch_repos,
        data_properties: dataproperty_vec,
        backup_windows: backupwindow_vec,
        retention_policies: retentions_vec,
        workloads: new_workloads,
    };

    let save_name = cli.save_file.split(".").collect::<Vec<&str>>();

    let file_name = format!("{}.json", save_name[0]);
    let mut json_file = fs::File::create(file_name)?;
    let vse_string = serde_json::to_string(&new_vse)?;
    json_file.write(vse_string.as_bytes())?;

    let toml_file_name = format!("{}.yaml", save_name[0]);
    let mut toml_file = fs::File::create(toml_file_name)?;
    let vse_toml_string = serde_yaml::to_string(&new_vse)?;
    toml_file.write(vse_toml_string.as_bytes())?;

    if cli.print {
        println!("{:#?}", new_vse);
    }

    println!("{}", "Complete".green());

    Ok(())
}
