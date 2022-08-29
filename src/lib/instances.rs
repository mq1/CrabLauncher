// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use druid::{im::Vector, Data};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};
use strum_macros::Display;

use color_eyre::eyre::{eyre, Result};

use super::BASE_DIR;

const INSTANCES_DIR: Lazy<PathBuf> = Lazy::new(|| BASE_DIR.join("instances"));

#[derive(Display, Serialize, Deserialize, Clone, Data, PartialEq, Eq)]
pub enum InstanceType {
    Vanilla,
    Fabric,
    Forge,
}

#[derive(Serialize, Deserialize, Clone, Data)]
pub struct InstanceInfo {
    pub instance_type: InstanceType,
    pub minecraft_version: String,
}

fn read_info(instance_name: &str) -> Result<InstanceInfo> {
    let path = INSTANCES_DIR.join(instance_name).join("instance.toml");
    let content = fs::read_to_string(path)?;
    let info: InstanceInfo = toml::from_str(&content)?;

    Ok(info)
}

pub fn list() -> Result<Vector<(String, InstanceInfo)>> {
    fs::create_dir_all(INSTANCES_DIR.as_path())?;

    let mut instances = Vector::new();
    for entry in fs::read_dir(INSTANCES_DIR.as_path())? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            let file_name = path.file_name().ok_or(eyre!("Could not get file name"))?;
            let file_name = file_name
                .to_str()
                .ok_or(eyre!("Could not convert file name to string"))?;

            let info = read_info(file_name)?;
            instances.push_back((file_name.to_string(), info));
        }
    }

    Ok(instances)
}

pub fn new() -> Result<()> {
    Ok(())
}
