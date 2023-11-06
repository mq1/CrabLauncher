// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuelquarneti@protonmail.com>
// SPDX-License-Identifier: GPL-3.0-only

use poll_promise::Promise;
use serde::Deserialize;

#[derive(Default)]
pub struct VanillaInstaller {
    pub versions: Option<Promise<Vec<String>>>,
    pub selected_version: String,
    pub name: String,
}

#[derive(Deserialize)]
struct Version {
    id: String,
}

#[derive(Deserialize)]
struct Response {
    versions: Vec<Version>,
}

impl VanillaInstaller {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn fetch_versions(&mut self) {
        if self.versions.is_some() {
            return;
        }

        let (sender, promise) = Promise::new();
        let request =
            ehttp::Request::get("https://piston-meta.mojang.com/mc/game/version_manifest_v2.json");
        ehttp::fetch(request, move |result| match result {
            Ok(response) => {
                let response = serde_json::from_slice::<Response>(&response.bytes).unwrap();
                let versions = response
                    .versions
                    .into_iter()
                    .map(|version| version.id)
                    .collect::<Vec<String>>();
                sender.send(versions);
            }
            Err(error) => {
                println!("Error: {}", error);
            }
        });
        self.versions = Some(promise);
    }
}
