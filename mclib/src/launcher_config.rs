// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use std::{fs, path::PathBuf};

use anyhow::Result;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use super::BASE_DIR;

const LAUNCHER_CONFIG_PATH: Lazy<PathBuf> = Lazy::new(|| BASE_DIR.join("config.toml"));

#[derive(Serialize, Deserialize)]
pub struct LauncherConfig {
    pub automatically_check_for_updates: bool,
    pub automatically_update_jvm: bool,
    pub automatically_optimize_jvm_arguments: bool,
    pub jvm_memory: String,
}

impl Default for LauncherConfig {
    fn default() -> Self {
        Self {
            automatically_check_for_updates: true,
            automatically_update_jvm: true,
            automatically_optimize_jvm_arguments: true,
            jvm_memory: "2G".to_string(),
        }
    }
}

pub fn write(config: &LauncherConfig) -> Result<()> {
    let content = toml::to_string_pretty(config)?;
    fs::write(LAUNCHER_CONFIG_PATH.as_path(), content)?;

    Ok(())
}

pub fn read() -> Result<LauncherConfig> {
    if !LAUNCHER_CONFIG_PATH.exists() {
        let default = LauncherConfig::default();
        write(&default)?;

        return Ok(default);
    }

    let content = fs::read_to_string(LAUNCHER_CONFIG_PATH.as_path())?;
    let config: LauncherConfig = toml::from_str(&content)?;

    Ok(config)
}

pub fn reset() -> Result<LauncherConfig> {
    let default = LauncherConfig::default();
    write(&default)?;

    Ok(default)
}
