// SPDX-FileCopyrightText: 2023 Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use std::{fmt::Display, fs};

use anyhow::Result;
use serde::Deserialize;

use crate::util::{runtime_manager, DownloadItem, AGENT};

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Version {
    pub id: String,
    pub url: String,
    pub sha1: String,
}

impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

pub async fn get_versions() -> Result<Vec<Version>> {
    #[derive(Deserialize)]
    struct Response {
        versions: Vec<Version>,
    }

    let resp = AGENT
        .get("https://piston-meta.mojang.com/mc/game/version_manifest_v2.json")
        .call()?
        .into_json::<Response>()?;

    Ok(resp.versions)
}

impl Version {
    pub fn install(&self) -> Result<Vec<DownloadItem>> {
        let mut download_items = Vec::new();

        let jvm_assets = runtime_manager::get_assets_info("17")?;
        if !runtime_manager::is_updated(&jvm_assets)? {
            let path = jvm_assets.get_path();
            if path.exists() {
                fs::remove_dir_all(path)?;
            }

            download_items.push(jvm_assets.get_download_item());
        }

        Ok(download_items)
    }
}
