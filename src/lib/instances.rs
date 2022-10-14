// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use std::{path::PathBuf, process::Stdio};

use druid::{im::Vector, Data};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use strum_macros::Display;
use tokio::{
    fs,
    io::{AsyncBufReadExt, BufReader},
    process::Command,
};

use color_eyre::eyre::Result;

use crate::{
    lib::{accounts, launcher_config, minecraft_assets::ASSETS_DIR},
    AppState, View,
};

use super::{
    minecraft_version_manifest::Version, minecraft_version_meta, runtime_manager, BASE_DIR,
};

// https://github.com/brucethemoose/Minecraft-Performance-Flags-Benchmarks
const OPTIMIZED_FLAGS: &str = "-XX:+UnlockExperimentalVMOptions -XX:+UnlockDiagnosticVMOptions -XX:+AlwaysActAsServerClassMachine -XX:+AlwaysPreTouch -XX:+DisableExplicitGC -XX:+UseNUMA -XX:NmethodSweepActivity=1 -XX:ReservedCodeCacheSize=400M -XX:NonNMethodCodeHeapSize=12M -XX:ProfiledCodeHeapSize=194M -XX:NonProfiledCodeHeapSize=194M -XX:-DontCompileHugeMethods -XX:MaxNodeLimit=240000 -XX:NodeLimitFudgeFactor=8000 -XX:+UseVectorCmov -XX:+PerfDisableSharedMem -XX:+UseFastUnorderedTimeStamps -XX:+UseCriticalJavaThreadPriority -XX:ThreadPriorityPolicy=1 -XX:AllocatePrefetchStyle=3 -XX:+UseShenandoahGC -XX:ShenandoahGCMode=iu -XX:ShenandoahGuaranteedGCInterval=1000000 -XX:AllocatePrefetchStyle=1";

const INSTANCES_DIR: Lazy<PathBuf> = Lazy::new(|| BASE_DIR.join("instances"));

#[derive(Display, Serialize, Deserialize, Clone, Data, PartialEq, Eq, Default)]
pub enum InstanceType {
    #[default]
    Vanilla,
    Fabric,
    Forge,
}

#[derive(Serialize, Deserialize, Clone, Data)]
pub struct InstanceInfo {
    pub instance_type: InstanceType,
    pub minecraft_version: String,
    pub jre_version: String,
}

impl Default for InstanceInfo {
    fn default() -> Self {
        Self {
            instance_type: InstanceType::default(),
            minecraft_version: "".to_string(),
            jre_version: "latest".to_string(),
        }
    }
}

#[derive(Clone, Data)]
pub struct Instance {
    pub name: String,
    pub info: InstanceInfo,
}

impl Instance {
    pub fn get_path(&self) -> PathBuf {
        INSTANCES_DIR.join(&self.name)
    }
}

async fn read_info(instance_name: &str) -> Result<InstanceInfo> {
    let path = INSTANCES_DIR.join(instance_name).join("instance.toml");
    let content = fs::read_to_string(path).await?;
    let info: InstanceInfo = toml::from_str(&content)?;

    Ok(info)
}

pub async fn list() -> Result<Vector<Instance>> {
    let mut instances = Vector::new();

    fs::create_dir_all(INSTANCES_DIR.as_path()).await?;
    let mut entries = fs::read_dir(INSTANCES_DIR.as_path()).await?;

    while let Some(entry) = entries.next_entry().await? {
        if !entry.path().is_dir() {
            continue;
        }

        let instance_name = entry.file_name().into_string().unwrap();
        let info = read_info(&instance_name).await?;

        instances.push_back(Instance {
            name: instance_name,
            info,
        });
    }

    Ok(instances)
}

pub async fn new(instance_name: &str, minecraft_version: &Version) -> Result<()> {
    let instance_dir = INSTANCES_DIR.join(instance_name);
    fs::create_dir_all(&instance_dir).await?;

    let info = InstanceInfo {
        minecraft_version: minecraft_version.id.clone(),
        ..Default::default()
    };

    let path = instance_dir.join("instance.toml");
    let content = toml::to_string_pretty(&info)?;
    fs::write(&path, content).await?;

    Ok(())
}

pub async fn remove(instance: Instance) -> Result<()> {
    let instance_dir = INSTANCES_DIR.join(instance.name);
    fs::remove_dir_all(&instance_dir).await?;

    Ok(())
}

pub async fn launch(instance: Instance, event_sink: druid::ExtEventSink) -> Result<()> {
    println!("Launching instance {}", instance.name);

    let account = accounts::get_active().await?.unwrap();

    let config = launcher_config::read().await?;

    let version = minecraft_version_meta::get(&instance.info.minecraft_version).await?;

    let jre_version = if instance.info.jre_version == "latest" {
        runtime_manager::fetch_available_releases()
            .await?
            .most_recent_feature_release
    } else {
        instance.info.jre_version.parse()?
    };

    /*
    let is_updated = runtime_manager::is_updated(&jre_version).await?;
    if !is_updated {
        println!("Installing JRE {}", jre_version);
        runtime_manager::install(&jre_version).await?;
    }
    */

    let java_path = runtime_manager::get_java_path(&jre_version).await?;

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
        account.mc_id,
        "--accessToken".to_string(),
        account.mc_access_token,
        "--clientId".to_string(),
        format!("ice-launcher/{}", env!("CARGO_PKG_VERSION")),
        "--userType".to_string(),
        "mojang".to_string(),
        "--versionType".to_string(),
        instance.info.instance_type.to_string(),
    ];

    let mut cmd = Command::new(java_path);

    cmd.stdout(Stdio::piped());

    let mut child = cmd
        .current_dir(instance.get_path())
        .args(jvm_args)
        .arg(version.main_class)
        .args(game_args)
        .spawn()
        .expect("failed to spawn command");

    let stdout = child
        .stdout
        .take()
        .expect("child did not have a handle to stdout");

    let mut reader = BufReader::new(stdout).lines();

    tokio::spawn(async move {
        let status = child
            .wait()
            .await
            .expect("child process encountered an error");

        println!("child status was: {}", status);

        event_sink.add_idle_callback(move |data: &mut AppState| {
            data.current_view = View::Instances;
        })
    });

    while let Some(line) = reader.next_line().await? {
        println!("Line: {}", line);
    }
    Ok(())
}
