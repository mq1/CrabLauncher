// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use std::{fs::File, path::PathBuf};

use anyhow::Result;
use once_cell::sync::Lazy;
use serde::Deserialize;
use url::Url;

use super::{
    minecraft_assets::AssetIndexInfo,
    minecraft_libraries::{Libraries, LibrariesExt, LIBRARIES_DIR},
    DownloadItem, HashAlgorithm, BASE_DIR,
};

pub const META_DIR: Lazy<PathBuf> = Lazy::new(|| BASE_DIR.join("meta"));

#[derive(Deserialize)]
#[serde(untagged)]
pub enum Value {
    Single(String),
    Multiple(Vec<String>),
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum Argument {
    Simple(String),
    Complex { value: Value },
}

#[derive(Deserialize)]
pub struct Arguments {
    pub game: Vec<Argument>,
    pub jvm: Vec<Argument>,
}

#[derive(Deserialize)]
pub struct Download {
    pub sha1: String,
    pub size: usize,
    pub url: Url,
}

#[derive(Deserialize)]
pub struct Downloads {
    pub client: Download,
}

#[derive(Deserialize)]
pub struct MinecraftVersionMeta {
    pub arguments: Arguments,
    #[serde(rename = "assetIndex")]
    pub asset_index: AssetIndexInfo,
    pub assets: String,
    pub id: String,
    pub downloads: Downloads,
    pub libraries: Libraries,
    #[serde(rename = "mainClass")]
    pub main_class: String,
}

impl MinecraftVersionMeta {
    fn get_client_path(&self) -> PathBuf {
        LIBRARIES_DIR
            .join("com")
            .join("mojang")
            .join("minecraft")
            .join(&self.id)
            .join(format!("minecraft-{}-client.jar", self.id))
    }

    pub fn get_download_items(&self) -> Result<Vec<DownloadItem>> {
        let mut items = Vec::new();

        // Assets
        let asset_index = self.asset_index.get()?;
        items.append(&mut asset_index.get_objects_download_items());

        // Libraries
        items.append(&mut self.libraries.get_download_items());

        // Client
        items.push(DownloadItem {
            url: self.downloads.client.url.clone(),
            path: self.get_client_path(),
            hash: (self.downloads.client.sha1.clone(), HashAlgorithm::Sha1),
        });

        Ok(items)
    }

    pub fn get_classpath(&self) -> String {
        #[cfg(target_os = "windows")]
        let separator = ";";

        #[cfg(not(target_os = "windows"))]
        let separator = ":";

        let mut jars = self
            .libraries
            .get_valid_libraries()
            .iter()
            .map(|l| l.get_path().to_string_lossy().to_string())
            .collect::<Vec<String>>();

        jars.push(self.get_client_path().to_string_lossy().to_string());

        jars.join(separator)
    }
}

pub fn get(version_id: &str) -> Result<MinecraftVersionMeta> {
    let path = META_DIR
        .join("net.minecraft")
        .join(format!("{}.json", version_id));

    let file = File::open(&path)?;
    let meta = serde_json::from_reader(file)?;

    Ok(meta)
}
