// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuelquarneti@protonmail.com>
// SPDX-License-Identifier: GPL-3.0-only

use crate::BASE_DIR;
use serde::Deserialize;
use std::fs;
use std::io::Write;
use std::sync::Arc;

const VERSION_MANIFEST_URL: &str =
    "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json";

#[derive(Deserialize, Debug, Clone)]
pub struct Latest {
    pub release: String,
    pub snapshot: String,
}

#[derive(Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Version {
    pub id: String,
    r#type: String,
    url: String,
    time: String,
    release_time: String,
    sha1: String,
    compliance_level: u8,
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.id)
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct VersionManifest {
    pub latest: Latest,
    pub versions: Vec<Version>,
}

impl VersionManifest {
    async fn _fetch() -> Result<Self, anyhow::Error> {
        let resp = reqwest::get(VERSION_MANIFEST_URL).await?;

        // write the response to a file
        let path = BASE_DIR.join("meta").join("version_manifest_v2.json");
        fs::create_dir_all(path.parent().unwrap())?;
        let bytes = resp.bytes().await?;
        fs::write(path, &bytes)?;

        let resp = serde_json::from_slice(&bytes)?;

        Ok(resp)
    }

    pub async fn fetch() -> Result<Self, Arc<anyhow::Error>> {
        Self::_fetch().await.map_err(Arc::new)
    }
}
