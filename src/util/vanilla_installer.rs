// SPDX-FileCopyrightText: 2023 Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use std::{fmt::Display, fs};

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::{
    util::{runtime_manager, DownloadItem, AGENT},
    META_DIR,
};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
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
    #[derive(Deserialize, Serialize)]
    struct Response {
        versions: Vec<Version>,
    }

    let resp = AGENT
        .get("https://piston-meta.mojang.com/mc/game/version_manifest_v2.json")
        .call()?
        .into_string()?;

    // save the response to a file
    let path = META_DIR.join("version_manifest_v2.json");
    fs::write(path, &resp)?;

    let resp: Response = serde_json::from_str(&resp)?;

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
