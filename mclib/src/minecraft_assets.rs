// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use std::{collections::HashMap, fs::File, path::PathBuf};

use anyhow::Result;
use once_cell::sync::Lazy;
use serde::Deserialize;

use super::{DownloadItem, HashAlgorithm, BASE_DIR};

const ASSETS_DOWNLOAD_ENDPOINT: &str = "https://resources.download.minecraft.net";

pub static ASSETS_DIR: Lazy<PathBuf> = Lazy::new(|| BASE_DIR.join("assets"));
static INDEXES_DIR: Lazy<PathBuf> = Lazy::new(|| ASSETS_DIR.join("indexes"));
static OBJECTS_DIR: Lazy<PathBuf> = Lazy::new(|| ASSETS_DIR.join("objects"));

#[derive(Deserialize)]
pub struct AssetIndexInfo {
    pub id: String,
    pub sha1: String,
    pub size: usize,
    #[serde(rename = "totalSize")]
    pub total_size: Option<usize>,
    pub url: String,
}

impl AssetIndexInfo {
    fn get_path(&self) -> PathBuf {
        INDEXES_DIR.join(format!("{}.json", &self.id))
    }

    fn download(&self) -> Result<()> {
        let item = DownloadItem {
            url: self.url.clone(),
            path: self.get_path(),
            hash: (self.sha1.clone(), HashAlgorithm::Sha1),
        };

        item.download()
    }

    pub fn get(&self) -> Result<AssetIndex> {
        let path = self.get_path();

        if !path.exists() {
            self.download()?;
        }

        let file = File::open(&path)?;
        let index = serde_json::from_reader(file)?;

        Ok(index)
    }
}

#[derive(Deserialize)]
struct Object {
    pub hash: String,
}

impl Object {
    fn get_url(&self) -> String {
        format!(
            "{}/{}/{}",
            ASSETS_DOWNLOAD_ENDPOINT,
            &self.hash[0..2],
            &self.hash
        )
    }

    fn get_path(&self) -> PathBuf {
        OBJECTS_DIR.join(&self.hash[..2]).join(&self.hash)
    }

    pub fn get_download_item(&self) -> DownloadItem {
        DownloadItem {
            url: self.get_url(),
            path: self.get_path(),
            hash: (self.hash.clone(), HashAlgorithm::Sha1),
        }
    }
}

#[derive(Deserialize)]
pub struct AssetIndex {
    #[serde(rename = "objects")]
    objects: HashMap<String, Object>,
}

impl AssetIndex {
    pub fn get_objects_download_items(&self) -> Vec<DownloadItem> {
        self.objects
            .values()
            .map(|object| object.get_download_item())
            .collect()
    }
}
