// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuelquarneti@protonmail.com>
// SPDX-License-Identifier: GPL-3.0-only

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
            name: "My Instance".to_string(),
            optimize_jvm: true,
            memory: "4G".to_string(),
        }
    }
}
