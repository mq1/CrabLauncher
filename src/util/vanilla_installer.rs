// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuq01@pm.me>
// SPDX-License-Identifier: GPL-3.0-only

use std::{collections::HashMap, fs};

use anyhow::Result;
use serde::Deserialize;

use crate::{
    util::{download_json, runtime_manager, DownloadItem, Hash, HashAlgorithm},
    ASSETS_DIR, META_DIR,
};

#[derive(Deserialize)]
struct VersionManifest {
    versions: Vec<Version>,
}

#[derive(Deserialize)]
pub struct Version {
    id: String,
    url: String,
    sha1: String,
}

pub fn get_versions() -> Result<Vec<String>> {
    let resp = download_json::<VersionManifest>(&DownloadItem {
        url: "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json".to_string(),
        path: META_DIR.join("version_manifest_v2.json.new"),
        hash: None,
    })?;

    fs::rename(
        META_DIR.join("version_manifest_v2.json.new"),
        META_DIR.join("version_manifest_v2.json"),
    )?;

    let versions = resp
        .versions
        .into_iter()
        .map(|v| v.id)
        .collect::<Vec<String>>();

    Ok(versions)
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

#[derive(Deserialize)]
pub struct AssetIndexMeta {
    id: String,
    sha1: String,
    url: String,
}

#[derive(Deserialize)]
pub struct VersionMeta {
    #[serde(rename = "assetIndex")]
    asset_index: AssetIndexMeta,
}

#[derive(Deserialize)]
pub struct Object {
    hash: String,
}

#[derive(Deserialize)]
pub struct AssetIndex {
    objects: HashMap<String, Object>,
}

pub fn download_version(id: &str) -> Result<(Vec<DownloadItem>, usize)> {
    let version_manifest = {
        let path = META_DIR.join("version_manifest_v2.json");
        let contents = fs::read_to_string(path)?;
        serde_json::from_str::<VersionManifest>(&contents)?
    };

    let version = version_manifest
        .versions
        .into_iter()
        .find(|v| v.id == id)
        .unwrap();

    // download version meta
    let version_meta = download_json::<VersionMeta>(&DownloadItem {
        url: version.url,
        path: META_DIR.join("versions").join(format!("{}.json", id)),
        hash: Some(Hash {
            hash: version.sha1,
            function: HashAlgorithm::Sha1,
        }),
    })?;

    let asset_index = download_json::<AssetIndex>(&DownloadItem {
        url: version_meta.asset_index.url,
        path: ASSETS_DIR
            .join("indexes")
            .join(format!("{}.json", version_meta.asset_index.id)),
        hash: Some(Hash {
            hash: version_meta.asset_index.sha1,
            function: HashAlgorithm::Sha1,
        }),
    })?;

    let mut download_items = Vec::new();

    for value in asset_index.objects.into_values() {
        let hash = Hash {
            hash: value.hash,
            function: HashAlgorithm::Sha1,
        };

        let path = ASSETS_DIR.join("objects").join(&hash.get_path());

        if !path.exists() {
            download_items.push(DownloadItem {
                url: format!(
                    "https://resources.download.minecraft.net/{}",
                    hash.get_path()
                ),
                path,
                hash: Some(hash),
            });
        }
    }

    let len = download_items.len();

    Ok((download_items, len))
}
