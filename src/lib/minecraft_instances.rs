// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use std::{path::PathBuf, fs::{self, File}, collections::HashMap};

use color_eyre::eyre::Result;
use once_cell::sync::Lazy;
use serde::Deserialize;
use url::Url;

use super::{download_file, BASE_DIR};

const ASSETS_DOWNLOAD_ENDPOINT: &str = "https://resources.download.minecraft.net";

static ASSETS_DIR: Lazy<PathBuf> = Lazy::new(|| BASE_DIR.join("instances"));
static INDEXES_DIR: Lazy<PathBuf> = Lazy::new(|| ASSETS_DIR.join("indexes"));
static OBJECTS_DIR: Lazy<PathBuf> = Lazy::new(|| ASSETS_DIR.join("objects"));


#[derive(Deserialize)]
struct Object {
    hash: String,
    size: i32,
}

pub fn install_assets(index_url: &str) -> Result<()> {
    let url = Url::parse(index_url)?;
    let index_file_name = url.path_segments().unwrap().last().unwrap();
    let index_path = INDEXES_DIR.join(index_file_name);

    download_file(index_url, index_path.to_str().unwrap())?;

    // parse index file
    let index = File::open(index_path)?;
    let index: HashMap<String, Object> = serde_json::from_reader(index)?;

    // download all objects
    for object in index.values() {
        let object_path = OBJECTS_DIR.join(&object.hash[..2]).join(&object.hash);
        let object_url = format!("{}/{}/{}", ASSETS_DOWNLOAD_ENDPOINT, &object.hash[..2], &object.hash);

        if !object_path.exists() {
            fs::create_dir_all(object_path.parent().unwrap())?;
            download_file(&object_url, object_path.to_str().unwrap())?;
        }
    }

    Ok(())
}
