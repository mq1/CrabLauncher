// SPDX-FileCopyrightText: 2023 Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use std::{fmt::Display, fs, path::PathBuf};

use anyhow::Result;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use time::PrimitiveDateTime;

use crate::{util, BASE_DIR};

pub static INSTANCES_DIR: Lazy<PathBuf> = Lazy::new(|| {
    let dir = BASE_DIR.join("instances");
    fs::create_dir_all(&dir).unwrap();

    dir
});

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InstanceInfo {
    last_played: Option<PrimitiveDateTime>,
    installer: String,
    version_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Instance {
    pub name: String,
    pub info: InstanceInfo,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Instances {
    pub list: Vec<Instance>,
}

impl Instances {
    fn sort(&mut self) {
        self.list.sort_by(|a, b| {
            let a = a.info.last_played.unwrap_or_else(|| PrimitiveDateTime::MIN);
            let b = b.info.last_played.unwrap_or_else(|| PrimitiveDateTime::MIN);

            b.cmp(&a)
        });
    }

    pub fn load() -> Result<Self> {
        if !INSTANCES_DIR.exists() {
            fs::create_dir(&*INSTANCES_DIR)?;

            return Ok(Self { list: Vec::new() });
        }

        let list = fs::read_dir(&*INSTANCES_DIR)?
            .filter_map(|entry| {
                let entry = entry.ok()?;

                // Skip non-directories
                if !entry.file_type().ok()?.is_dir() {
                    return None;
                }

                let path = entry.path();
                let name: String = path.file_name()?.to_str()?.to_string();

                let info_path = INSTANCES_DIR.join(&name).join("instance.toml");
                let info = fs::read_to_string(info_path).ok()?;
                let info = toml::from_str(&info).ok()?;

                let instance = Instance { name, info };

                Some(instance)
            })
            .collect();

        let mut instances = Self { list };
        instances.sort();

        Ok(instances)
    }

    pub fn new(
        &mut self,
        name: String,
        installer: String,
        version: util::vanilla_installer::Version,
    ) -> Result<()> {
        let path = INSTANCES_DIR.join(&name);
        fs::create_dir(&path)?;

        let info = InstanceInfo {
            last_played: None,
            installer,
            version_id: version.id,
        };
        let info_str = toml::to_string_pretty(&info)?;
        fs::write(path.join("instance.toml"), info_str)?;

        self.list.push(Instance { name, info });

        Ok(())
    }
}

impl Display for Instance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
