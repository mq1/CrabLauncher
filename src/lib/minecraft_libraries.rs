// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use std::path::PathBuf;

use once_cell::sync::Lazy;
use serde::Deserialize;

use super::{
    minecraft_rules::{is_rule_list_valid, Rule},
    BASE_DIR,
};

pub static LIBRARIES_DIR: Lazy<PathBuf> = Lazy::new(|| BASE_DIR.join("libraries"));

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
const NATIVES_STRING: &str = "natives-linux";

#[cfg(all(target_os = "macos", target_arch = "x86_64"))]
const NATIVES_STRING: &str = "natives-macos";

#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
const NATIVES_STRING: &str = "natives-macos-arm64";

#[cfg(all(target_os = "windows", target_arch = "x86_64"))]
const NATIVES_STRING: &str = "natives-windows";

#[derive(Deserialize)]
pub struct Artifact {
    pub path: String,
    pub sha1: String,
    pub size: usize,
    pub url: String,
}

impl Artifact {
    fn is_valid(&self) -> bool {
        if self.path.contains("natives") && !self.path.contains(NATIVES_STRING) {
            return false;
        }

        #[cfg(not(target_arch = "x86_64"))]
        if self.path.contains("x86_64") {
            return false;
        }

        #[cfg(not(target_arch = "aarch64"))]
        if self.path.contains("aarch_64") {
            return false;
        }

        return true;
    }

    pub fn get_path(&self) -> PathBuf {
        LIBRARIES_DIR.join(&self.path)
    }
}

#[derive(Deserialize)]
pub struct LibraryDownloads {
    pub artifact: Artifact,
    rules: Option<Vec<Rule>>,
}

#[derive(Deserialize)]
pub struct Library {
    pub downloads: LibraryDownloads,
}

impl Library {
    pub fn is_valid(&self) -> bool {
        if !self.downloads.artifact.is_valid() {
            return false;
        }

        if let Some(rules) = &self.downloads.rules {
            return is_rule_list_valid(rules);
        }

        return true;
    }

    pub fn get_path(&self) -> String {
        LIBRARIES_DIR
            .join(&self.downloads.artifact.path)
            .to_string_lossy()
            .to_string()
    }
}
