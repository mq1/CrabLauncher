// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use std::{fs, path::PathBuf, process::Command};

use color_eyre::Result;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use strum_macros::Display;

use crate::lib::{accounts, launcher_config, minecraft_assets::ASSETS_DIR};

use super::{
    minecraft_libraries::LibrariesExt, minecraft_version_manifest::Version, minecraft_version_meta,
    runtime_manager, BASE_DIR,
};

// https://github.com/brucethemoose/Minecraft-Performance-Flags-Benchmarks
const OPTIMIZED_FLAGS: &str = "-XX:+UnlockExperimentalVMOptions -XX:+UnlockDiagnosticVMOptions -XX:+AlwaysActAsServerClassMachine -XX:+AlwaysPreTouch -XX:+DisableExplicitGC -XX:+UseNUMA -XX:NmethodSweepActivity=1 -XX:ReservedCodeCacheSize=400M -XX:NonNMethodCodeHeapSize=12M -XX:ProfiledCodeHeapSize=194M -XX:NonProfiledCodeHeapSize=194M -XX:-DontCompileHugeMethods -XX:MaxNodeLimit=240000 -XX:NodeLimitFudgeFactor=8000 -XX:+UseVectorCmov -XX:+PerfDisableSharedMem -XX:+UseFastUnorderedTimeStamps -XX:+UseCriticalJavaThreadPriority -XX:ThreadPriorityPolicy=1 -XX:AllocatePrefetchStyle=3 -XX:+UseShenandoahGC -XX:ShenandoahGCMode=iu -XX:ShenandoahGuaranteedGCInterval=1000000 -XX:AllocatePrefetchStyle=1";

const INSTANCES_DIR: Lazy<PathBuf> = Lazy::new(|| BASE_DIR.join("instances"));

#[derive(Display, Serialize, Deserialize, Debug, Clone)]
pub enum InstanceType {
    Vanilla,
    Fabric,
    Forge,
    ModrinthModpack,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct InstanceInfo {
    pub instance_type: InstanceType,
    pub minecraft_version: String,
}

#[derive(Debug, Clone)]
pub struct Instance {
    pub name: String,
    pub info: InstanceInfo,
}

impl Instance {
    pub fn get_path(&self) -> PathBuf {
        INSTANCES_DIR.join(&self.name)
    }
}

fn read_info(instance_name: &str) -> Result<InstanceInfo> {
    let path = INSTANCES_DIR.join(instance_name).join("instance.toml");
    let content = fs::read_to_string(path)?;
    let info: InstanceInfo = toml::from_str(&content)?;

    Ok(info)
}

pub fn list() -> Result<Vec<Instance>> {
    let mut instances = Vec::new();

    fs::create_dir_all(INSTANCES_DIR.as_path())?;
    let mut entries = fs::read_dir(INSTANCES_DIR.as_path())?;

    while let Some(entry) = entries.next() {
        let entry = entry?;
        let path = entry.path();

        if !path.is_dir() {
            continue;
        }

        let name = path.file_name().unwrap().to_str().unwrap().to_string();
        let info = read_info(&name)?;

        instances.push(Instance { name, info });
    }

    Ok(instances)
}

pub fn new(instance_name: String, minecraft_version: Version) -> Result<()> {
    let meta = minecraft_version.get_meta()?;

    let asset_index = meta.asset_index.get()?;
    asset_index.download_objects()?;

    let libraries = meta.libraries.get_valid_libraries();
    libraries.download()?;

    meta.download_client()?;

    let jvm_assets = runtime_manager::get_assets_info("17")?;
    if !runtime_manager::is_updated(&jvm_assets)? {
        runtime_manager::update(&jvm_assets)?;
    }

    let instance_dir = INSTANCES_DIR.join(instance_name);
    fs::create_dir_all(&instance_dir)?;

    let info = InstanceInfo {
        minecraft_version: minecraft_version.id,
        instance_type: InstanceType::Vanilla,
    };

    let path = instance_dir.join("instance.toml");
    let content = toml::to_string_pretty(&info)?;
    fs::write(&path, content)?;
    let instances = list()?;

    Ok(())
}

pub fn remove(instance_name: &str) -> Result<()> {
    let instance_dir = INSTANCES_DIR.join(instance_name);
    fs::remove_dir_all(&instance_dir)?;

    Ok(())
}

pub fn launch(instance: Instance) -> Result<()> {
    let instance_name = instance.name.clone();
    let account = accounts::get_active()?.unwrap();
    let account = accounts::refresh(account)?;

    let config = launcher_config::read()?;

    let version = minecraft_version_meta::get(&instance.info.minecraft_version)?;

    if config.automatically_update_jvm {
        let jvm_assets = runtime_manager::get_assets_info("17")?;

        if !runtime_manager::is_updated(&jvm_assets)? {
            runtime_manager::update(&jvm_assets)?;
        }
    }

    let java_path = runtime_manager::get_java_path("17")?;

    let mut jvm_args = vec![
        "-Dminecraft.launcher.brand=ice-launcher".to_string(),
        format!("-Dminecraft.launcher.version={}", env!("CARGO_PKG_VERSION")),
        format!("-Xmx{}", config.jvm_memory),
        format!("-Xms{}", config.jvm_memory),
        "-cp".to_string(),
        version.get_classpath(),
    ];

    #[cfg(target_os = "macos")]
    jvm_args.push("-XstartOnFirstThread".to_string());

    if config.automatically_optimize_jvm_arguments {
        jvm_args.extend(
            OPTIMIZED_FLAGS
                .split_whitespace()
                .map(|s| s.to_string())
                .collect::<Vec<_>>(),
        );
    }

    let game_args = vec![
        "--username".to_string(),
        account.mc_username,
        "--version".to_string(),
        instance.info.minecraft_version.to_owned(),
        "--gameDir".to_string(),
        ".".to_string(),
        "--assetsDir".to_string(),
        ASSETS_DIR.to_string_lossy().to_string(),
        "--assetIndex".to_string(),
        version.assets,
        "--uuid".to_string(),
        account.mc_id.to_string(),
        "--accessToken".to_string(),
        account.mc_access_token,
        "--clientId".to_string(),
        format!("ice-launcher/{}", env!("CARGO_PKG_VERSION")),
        "--userType".to_string(),
        "mojang".to_string(),
        "--versionType".to_string(),
        instance.info.instance_type.to_string(),
    ];

    let mut child = Command::new(java_path)
        .current_dir(instance.get_path())
        .args(jvm_args)
        .arg(version.main_class)
        .args(game_args)
        .spawn()?;

    child.wait()?;
    Ok(())
}
