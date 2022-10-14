// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use std::{collections::HashMap, path::PathBuf};

use once_cell::sync::Lazy;
use serde::Deserialize;
use url::Url;

use super::BASE_DIR;

const ASSETS_DOWNLOAD_ENDPOINT: Lazy<Url> =
    Lazy::new(|| Url::parse("https://resources.download.minecraft.net").unwrap());

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
    pub url: Url,
}

#[derive(Deserialize)]
pub struct Object {
    pub hash: String,
    pub size: usize,
}

impl Object {
    pub fn get_path(&self) -> PathBuf {
        OBJECTS_DIR.join(&self.hash[..2]).join(&self.hash)
    }

    pub fn get_url(&self) -> Url {
        ASSETS_DOWNLOAD_ENDPOINT
            .join(&format!("{}/{}", &self.hash[..2], &self.hash))
            .unwrap()
    }
}

#[derive(Deserialize)]
pub struct AssetIndex {
    #[serde(rename = "objects")]
    pub objects: HashMap<String, Object>,
}

impl AssetIndexInfo {
    pub fn get_path(&self) -> PathBuf {
        INDEXES_DIR.join(&self.id).with_extension("json")
    }
}
