// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use std::{collections::HashMap, path::PathBuf};

use color_eyre::eyre::Result;
use once_cell::sync::Lazy;
use serde::Deserialize;
use smol::fs;
use url::Url;

use super::{download_file, BASE_DIR};

const ASSETS_DOWNLOAD_ENDPOINT: &str = "https://resources.download.minecraft.net";

static ASSETS_DIR: Lazy<PathBuf> = Lazy::new(|| BASE_DIR.join("assets"));
static INDEXES_DIR: Lazy<PathBuf> = Lazy::new(|| ASSETS_DIR.join("indexes"));
static OBJECTS_DIR: Lazy<PathBuf> = Lazy::new(|| ASSETS_DIR.join("objects"));

#[derive(Deserialize)]
pub struct AssetIndex {
    pub id: String,
    pub sha1: String,
    pub size: i32,
    #[serde(rename = "totalSize")]
    pub total_size: Option<i32>,
    pub url: String,
}

#[derive(Deserialize)]
struct Object {
    hash: String,
    size: i32,
}

#[derive(Deserialize)]
struct Index {
    #[serde(rename = "objects")]
    objects: HashMap<String, Object>,
}

pub async fn install(index: &AssetIndex) -> Result<()> {
    let results = [
        fs::create_dir_all(ASSETS_DIR.as_path()),
        fs::create_dir_all(INDEXES_DIR.as_path()),
        fs::create_dir_all(OBJECTS_DIR.as_path()),
    ];

    for result in results.into_iter() {
        result.await?;
    }

    let url = Url::parse(&index.url)?;
    let index_file_name = url.path_segments().unwrap().last().unwrap();
    let index_path = INDEXES_DIR.join(index_file_name);

    download_file(&index.url, &index_path, Some(&index.sha1)).await?;

    // parse index file
    let index = fs::read_to_string(index_path).await?;
    let index: Index = serde_json::from_str(&index)?;

    // download all objects
    for object in index.objects.values() {
        let object_path = OBJECTS_DIR.join(&object.hash[..2]).join(&object.hash);
        let object_url = format!(
            "{}/{}/{}",
            ASSETS_DOWNLOAD_ENDPOINT,
            &object.hash[..2],
            &object.hash
        );

        if !object_path.exists() {
            fs::create_dir_all(object_path.parent().unwrap()).await?;
            download_file(&object_url, &object_path, Some(&object.hash)).await?;
        }
    }

    Ok(())
}
