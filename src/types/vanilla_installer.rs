// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuelquarneti@protonmail.com>
// SPDX-License-Identifier: GPL-3.0-only

use chrono::Local;

pub struct VanillaInstaller {
    pub versions: Vec<String>,
    pub selected_version: Option<usize>,
    pub name: String,
    pub optimize_jvm: bool,
    pub memory: String,
}

impl Default for VanillaInstaller {
    fn default() -> Self {
        Self {
            versions: Vec::new(),
            selected_version: None,
            name: format!("Vanilla {}", Local::now().format("%Y-%m-%d %H:%M:%S")),
            optimize_jvm: true,
            memory: "4G".to_string(),
        }
    }
}
