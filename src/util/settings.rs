// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuq01@pm.me>
// SPDX-License-Identifier: GPL-3.0-only

use std::{fs, path::PathBuf};

use anyhow::Result;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use crate::BASE_DIR;

pub static PATH: Lazy<PathBuf> = Lazy::new(|| BASE_DIR.join("settings.toml"));

#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub check_for_updates: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            check_for_updates: true,
        }
    }
}

impl Settings {
    pub fn load() -> Result<Self> {
        if !PATH.exists() {
            return Ok(Self::default());
        }

        let settings = fs::read_to_string(&*PATH)?;
        let settings: Self = toml::from_str(&settings)?;
        Ok(settings)
    }

    pub fn save(&self) -> Result<()> {
        let settings = toml::to_string_pretty(self)?;
        fs::write(&*PATH, settings)?;
        Ok(())
    }
}
