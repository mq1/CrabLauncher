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

impl LauncherConfig {
    pub fn load() -> Result<Self> {
        if LAUNCHER_CONFIG_PATH.exists() {
            let content = fs::read_to_string(&*LAUNCHER_CONFIG_PATH)?;
            let config = toml::from_str(&content)?;
            Ok(config)
        } else {
            let config = Self::default();
            config.save()?;
            Ok(config)
        }
    }

    pub fn save(&self) -> Result<()> {
        let config = toml::to_string_pretty(self)?;
        fs::write(&*LAUNCHER_CONFIG_PATH, config)?;
        Ok(())
    }

    pub fn reset(&mut self) -> Result<()> {
        *self = Self::default();
        self.save()?;
        Ok(())
    }
}
