// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use color_eyre::Result;
use druid::{im::Vector, Data, Lens};
use isahc::AsyncReadResponseExt;
use serde::Deserialize;
use smol::fs;

use super::{
    download_file,
    minecraft_version_meta::{MinecraftVersionMeta, VERSIONS_DIR},
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

#[derive(Deserialize, Clone, Data, PartialEq, Eq, Lens)]
pub struct Version {
    pub id: String,
    #[serde(rename = "type")]
    pub version_type: VersionType,
    pub url: String,
    pub time: String,
    #[serde(rename = "releaseTime")]
    pub release_time: String,
    pub sha1: String,
    #[serde(rename = "complianceLevel")]
    pub compliance_level: i32,
}

impl Version {
    pub async fn install(&self) -> Result<()> {
        let version_dir = VERSIONS_DIR.join(&self.id);
        fs::create_dir_all(&version_dir).await?;

        let meta_path = version_dir.join("meta.json");
        download_file(&self.url, &meta_path, Some(&self.sha1)).await?;
        let meta = fs::read_to_string(meta_path).await?;
        let meta: MinecraftVersionMeta = serde_json::from_str(&meta)?;
        meta.install().await?;

        Ok(())
    }
}

#[derive(Deserialize, Clone, Data, PartialEq, Eq)]
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

pub async fn fetch_manifest() -> Result<MinecraftVersionManifest> {
    let manifest = isahc::get_async(VERSION_MANIFEST_URL).await?.json().await?;

    Ok(manifest)
}

pub async fn fetch_versions() -> Result<Vector<Version>> {
    let manifest = fetch_manifest().await?;
    let versions = Vector::from(manifest.versions);

    Ok(versions)
}
