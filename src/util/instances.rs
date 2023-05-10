// SPDX-FileCopyrightText: 2023 Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use std::{fs, path::PathBuf};

use anyhow::Result;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use time::PrimitiveDateTime;

use crate::BASE_DIR;

pub static INSTANCES_DIR: Lazy<PathBuf> = Lazy::new(|| BASE_DIR.join("instances"));

#[derive(Serialize, Deserialize)]
pub struct Instance {
    last_played: Option<PrimitiveDateTime>,
}

pub struct Instances {
    pub list: Vec<String>,
}

impl Instances {
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
                let path = path.file_name()?.to_str()?.to_string();
                Some(path)
            })
            .collect();

        Ok(Self { list })
    }

    pub fn new(name: &str) -> Result<()> {
        let path = INSTANCES_DIR.join(name);
        fs::create_dir(&path)?;

        let info = Instance { last_played: None };
        let info = toml::to_string_pretty(&info)?;
        fs::write(path.join("instance.toml"), info)?;

        Ok(())
    }

    pub fn get_info(&self, name: &str) -> Result<Instance> {
        let path = INSTANCES_DIR.join(name).join("instance.toml");
        let info = fs::read_to_string(path)?;
        let info: Instance = toml::from_str(&info)?;

        Ok(info)
    }
}
