// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use std::{
    fs::{self, File},
    path::PathBuf,
};

use color_eyre::Result;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use super::{download_file, minecraft_version_manifest, BASE_DIR, minecraft_assets};

const VERSIONS_DIR: Lazy<PathBuf> = Lazy::new(|| BASE_DIR.join("versions"));

#[derive(Serialize, Deserialize)]
pub struct MinecraftVersionMeta {
    #[serde(rename = "assetIndex")]
    pub asset_index: AssetIndex,
    pub assets: String,
}

#[derive(Serialize, Deserialize)]
pub struct AssetIndex {
    pub id: String,
    pub sha1: String,
    pub size: i32,
    #[serde(rename = "totalSize")]
    pub total_size: Option<i32>,
    pub url: String,
}

pub fn install(version: &minecraft_version_manifest::Version) -> Result<()> {
    let version_dir = VERSIONS_DIR.join(&version.id);
    fs::create_dir_all(&version_dir)?;

    let meta_path = version_dir.join("meta.json");
    download_file(&version.url, &meta_path)?;
    let file = File::open(&meta_path)?;
    let meta: MinecraftVersionMeta = serde_json::from_reader(file)?;

    minecraft_assets::install(&meta.asset_index.url)?;

    Ok(())
}
