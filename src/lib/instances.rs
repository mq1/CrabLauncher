// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use druid::{im::Vector, Data};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use smol::{fs, stream::StreamExt};
use std::{path::PathBuf, process::Command};
use strum_macros::Display;

use color_eyre::eyre::Result;

use super::{
    minecraft_assets::ASSETS_DIR, minecraft_version_manifest::Version, minecraft_version_meta, msa,
    runtime_manager, BASE_DIR,
};

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

    while let Some(entry) = entries.try_next().await? {
        if entry.path().is_dir() {
            let file_name = entry.file_name().to_string_lossy().to_string();
            let info = read_info(&file_name).await?;
            instances.push_back(Instance {
                name: file_name,
                info,
            });
        }
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

    minecraft_version.install().await?;

    Ok(())
}

pub async fn remove(instance: Instance) -> Result<()> {
    let instance_dir = INSTANCES_DIR.join(instance.name);
    fs::remove_dir_all(&instance_dir).await?;

    Ok(())
}

pub async fn launch(instance: Instance, account: msa::Account) -> Result<()> {
    println!("Launching instance {}", instance.name);

    let version = minecraft_version_meta::get(&instance.info.minecraft_version).await?;

    let jre_version = if instance.info.jre_version == "latest" {
        runtime_manager::fetch_available_releases()
            .await?
            .most_recent_feature_release
    } else {
        instance.info.jre_version.parse()?
    };

    let is_updated = runtime_manager::is_updated(&jre_version).await?;
    if !is_updated {
        println!("Installing JRE {}", jre_version);
        runtime_manager::install(&jre_version).await?;
    }

    let java_path = runtime_manager::get_java_path(&jre_version).await?;

    let mut game_args = Vec::new();
    for arg in &version.arguments.game {
        let arg = match arg {
            minecraft_version_meta::Argument::Simple(arg) => Some(arg),
            minecraft_version_meta::Argument::Complex { value } => None,
        };

        if let Some(arg) = arg {
            let arg = match arg.as_str() {
                "${auth_player_name}" => account.mc_username.to_owned(),
                "${version_name}" => instance.info.minecraft_version.to_string(),
                "${game_directory}" => INSTANCES_DIR
                    .join(&instance.name)
                    .to_string_lossy()
                    .to_string(),
                "${assets_root}" => ASSETS_DIR.to_string_lossy().to_string(),
                "${assets_index_name}" => version.assets.to_string(),
                "${auth_uuid}" => account.mc_id.to_owned(),
                "${auth_access_token}" => account.mc_access_token.to_owned(),
                "${clientid}" => format!("ice-launcher/{}", env!("CARGO_PKG_VERSION")),
                "${auth_xuid}" => "0".to_string(),
                "${user_type}" => "mojang".to_string(),
                "${version_type}" => instance.info.instance_type.to_string(),
                &_ => arg.to_string(),
            };

            game_args.push(arg);
        }
    }

    let mut jvm_args = vec![
        "-Djava.library.path=natives".to_string(),
        "-Dminecraft.launcher.brand=ice-launcher".to_string(),
        format!("-Dminecraft.launcher.version={}", env!("CARGO_PKG_VERSION")),
    ];

    #[cfg(target_os = "macos")]
    jvm_args.push("-XstartOnFirstThread".to_string());

    jvm_args.push("-cp".to_string());
    jvm_args.push(version.get_classpath());

    Command::new(java_path)
        .args(jvm_args)
        .arg(version.main_class)
        .args(game_args)
        .current_dir(INSTANCES_DIR.join(instance.name))
        .spawn()?;

    Ok(())
}
