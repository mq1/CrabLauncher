// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use std::path::PathBuf;

use color_eyre::eyre::Result;
use druid::{Data, Lens};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use tokio::fs;

use super::BASE_DIR;

const LAUNCHER_CONFIG_PATH: Lazy<PathBuf> = Lazy::new(|| BASE_DIR.join("config.toml"));

#[derive(Serialize, Deserialize, Data, Clone, Lens)]
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

impl AsRef<LauncherConfig> for LauncherConfig {
    fn as_ref(&self) -> &Self {
        self
    }
}

pub async fn write<L: AsRef<LauncherConfig>>(config: L) -> Result<()> {
    let content = toml::to_string_pretty(config.as_ref())?;
    fs::write(LAUNCHER_CONFIG_PATH.as_path(), content).await?;

    Ok(())
}

pub async fn read() -> Result<LauncherConfig> {
    if !LAUNCHER_CONFIG_PATH.exists() {
        let default = LauncherConfig::default();
        write(&default).await?;

        return Ok(default);
    }

    let content = fs::read_to_string(LAUNCHER_CONFIG_PATH.as_path()).await?;
    let config: LauncherConfig = toml::from_str(&content)?;

    Ok(config)
}
