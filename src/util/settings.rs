// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuelquarneti@protonmail.com>
// SPDX-License-Identifier: GPL-3.0-only

use std::fs;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::util::paths::SETTINGS_PATH;

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
        if !SETTINGS_PATH.exists() {
            return Ok(Self::default());
        }

        let settings = fs::read_to_string(&*SETTINGS_PATH)?;
        let settings: Self = toml::from_str(&settings)?;
        Ok(settings)
    }

    pub fn save(&self) -> Result<()> {
        let settings = toml::to_string_pretty(self)?;
        fs::write(&*SETTINGS_PATH, settings)?;
        Ok(())
    }
}
