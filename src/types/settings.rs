// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuelquarneti@protonmail.com>
// SPDX-License-Identifier: GPL-3.0-only

use crate::BASE_DIR;
use anyhow::Result;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

pub static SETTINGS_PATH: Lazy<PathBuf> = Lazy::new(|| BASE_DIR.join("settings.toml"));

#[derive(Serialize, Deserialize)]
pub struct Settings {
    check_for_updates: bool,
    java_path: String,
    java_memory: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            check_for_updates: true,
            java_path: "java".to_string(),
            java_memory: "4G".to_string(),
        }
    }
}

impl Settings {
    pub fn load() -> Result<Self> {
        let settings = if SETTINGS_PATH.exists() {
            let settings = fs::read_to_string(&*SETTINGS_PATH)?;
            toml::from_str(&settings)?
        } else {
            Self::default()
        };

        Ok(settings)
    }

    pub fn save(&self) -> Result<()> {
        let settings = toml::to_string(&self)?;
        fs::write(&*SETTINGS_PATH, settings)?;

        Ok(())
    }
}
