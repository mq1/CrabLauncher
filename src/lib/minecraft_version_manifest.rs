// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use core::fmt;
use std::{fs::File, path::PathBuf};

use anyhow::Result;
use serde::Deserialize;
use url::Url;

use super::{
    minecraft_version_meta::{MinecraftVersionMeta, META_DIR},
    DownloadItem, HashAlgorithm, HTTP_CLIENT,
};

const VERSION_MANIFEST_URL: &str =
    "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json";

#[derive(Deserialize)]
pub struct MinecraftVersionManifest {
    pub latest: Latest,
    pub versions: Vec<Version>,
}

#[derive(Deserialize)]
pub struct Latest {
    pub release: String,
    pub snapshot: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Version {
    pub id: String,
    #[serde(rename = "type")]
    version_type: VersionType,
    url: String,
    time: String,
    #[serde(rename = "releaseTime")]
    release_time: String,
    sha1: String,
    #[serde(rename = "complianceLevel")]
    compliance_level: i32,
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl Version {
    fn get_meta_path(&self) -> PathBuf {
        META_DIR
            .join("net.minecraft")
            .join(format!("{}.json", self.id))
    }

    fn get_meta_download_item(&self) -> Result<DownloadItem> {
        let url = Url::parse(&self.url)?;
        let path = self.get_meta_path();

        Ok(super::DownloadItem {
            url,
            path,
            hash: (self.sha1.clone(), HashAlgorithm::Sha1),
        })
    }

    pub fn get_meta(&self) -> Result<MinecraftVersionMeta> {
        self.get_meta_download_item()?.download()?;

        let path = self.get_meta_path();
        let file = File::open(&path)?;
        let meta = serde_json::from_reader(file)?;

        Ok(meta)
    }
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub enum VersionType {
    #[serde(rename = "old_alpha")]
    OldAlpha,
    #[serde(rename = "old_beta")]
    OldBeta,
    #[serde(rename = "release")]
    Release,
    #[serde(rename = "snapshot")]
    Snapshot,
}

fn fetch_manifest() -> Result<MinecraftVersionManifest> {
    let manifest = HTTP_CLIENT.get(VERSION_MANIFEST_URL).send()?.json()?;

    Ok(manifest)
}

pub fn fetch_versions() -> Result<Vec<Version>> {
    let manifest = fetch_manifest()?;
    let versions = manifest.versions;

    // Filter out versions < 1.19
    let mut filtered_versions = Vec::new();

    for version in versions {
        let stop = version.id == "1.19";

        filtered_versions.push(version);

        if stop {
            break;
        }
    }

    Ok(filtered_versions)
}
