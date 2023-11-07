// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuelquarneti@protonmail.com>
// SPDX-License-Identifier: GPL-3.0-only

use crate::AGENT;
use poll_promise::Promise;
use serde::Deserialize;
use std::thread;

pub struct VanillaInstaller {
    pub versions: Option<Promise<Vec<String>>>,
    pub selected_version: String,
    pub name: String,
}

impl Default for VanillaInstaller {
    fn default() -> Self {
        Self {
            versions: None,
            selected_version: "".to_string(),
            name: "My new instance".to_string(),
        }
    }
}
#[derive(Deserialize)]
struct Version {
    id: String,
}

#[derive(Deserialize)]
struct VersionManifest {
    versions: Vec<Version>,
}

impl VanillaInstaller {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn fetch_versions(&mut self) {
        let (sender, promise) = Promise::new();

        thread::spawn(move || {
            let version_manifest = AGENT
                .get("https://piston-meta.mojang.com/mc/game/version_manifest_v2.json")
                .call()
                .unwrap()
                .into_json::<VersionManifest>()
                .unwrap();

            let versions = version_manifest
                .versions
                .into_iter()
                .map(|version| version.id)
                .collect::<Vec<String>>();

            sender.send(versions);
        });

        self.versions = Some(promise);
    }
}
