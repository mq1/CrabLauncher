// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use druid::{im::Vector, Data};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use smol::{fs, stream::StreamExt};
use std::path::PathBuf;
use strum_macros::Display;

use color_eyre::eyre::Result;

use super::{minecraft_version_manifest::Version, minecraft_version_meta, BASE_DIR};

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
}

async fn read_info(instance_name: &str) -> Result<InstanceInfo> {
    let path = INSTANCES_DIR.join(instance_name).join("instance.toml");
    let content = fs::read_to_string(path).await?;
    let info: InstanceInfo = toml::from_str(&content)?;

    Ok(info)
}

pub async fn list() -> Result<Vector<(String, InstanceInfo)>> {
    let mut instances = Vector::new();

    fs::create_dir_all(INSTANCES_DIR.as_path()).await?;
    let mut entries = fs::read_dir(INSTANCES_DIR.as_path()).await?;

    while let Some(entry) = entries.try_next().await? {
        if entry.path().is_dir() {
            let file_name = entry.file_name().to_string_lossy().to_string();
            let info = read_info(&file_name).await?;
            instances.push_back((file_name, info));
        }
    }

    Ok(instances)
}

pub async fn new(instance_name: &str, minecraft_version: &Version) -> Result<()> {
    let instance_dir = INSTANCES_DIR.join(instance_name);
    fs::create_dir_all(&instance_dir).await?;

    let info = InstanceInfo {
        instance_type: InstanceType::Vanilla,
        minecraft_version: minecraft_version.id.clone(),
    };

    let path = instance_dir.join("instance.toml");
    let content = toml::to_string(&info)?;
    fs::write(&path, content).await?;

    minecraft_version_meta::install(minecraft_version).await?;

    Ok(())
}
