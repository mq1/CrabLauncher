// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuelquarneti@protonmail.com>
// SPDX-License-Identifier: GPL-3.0-only

use crate::message::VanillaInstallerMessage;

pub struct VanillaInstaller {
    available_versions: Vec<String>,
    selected_version: String,
    name: String,
}

impl VanillaInstaller {
    pub fn new() -> Self {
        Self {
            available_versions: Vec::new(),
            selected_version: "".to_string(),
            name: "My new Instance".to_string(),
        }
    }

    pub fn update(&mut self, vanilla_installer_message: VanillaInstallerMessage) {
        match vanilla_installer_message {
            VanillaInstallerMessage::ChangeName(name) => {
                self.name = name;
            }
        }
    }
}
