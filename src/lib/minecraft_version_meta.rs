// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use std::path::PathBuf;

use color_eyre::Result;
use once_cell::sync::Lazy;
use serde::Deserialize;
use smol::fs;

use super::{
    download_file,
    minecraft_assets::{self, AssetIndex},
    minecraft_libraries::{self, get_valid_artifacts, Artifact, Library},
    minecraft_version_manifest, BASE_DIR,
};

const VERSIONS_DIR: Lazy<PathBuf> = Lazy::new(|| BASE_DIR.join("versions"));

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
pub struct Downloads {
    pub client: Artifact,
}

#[derive(Deserialize)]
pub struct MinecraftVersionMeta {
    pub arguments: Arguments,
    #[serde(rename = "assetIndex")]
    pub asset_index: AssetIndex,
    pub assets: String,
    pub id: String,
    pub downloads: Downloads,
    pub libraries: Vec<Library>,
    #[serde(rename = "mainClass")]
    pub main_class: String,
}

impl MinecraftVersionMeta {
    fn get_client_path(&self) -> String {
        VERSIONS_DIR
            .join(&self.id)
            .join("client.jar")
            .to_string_lossy()
            .to_string()
    }

    pub fn get_classpath(&self) -> String {
        #[cfg(target_os = "windows")]
        let separator = ";";

        #[cfg(not(target_os = "windows"))]
        let separator = ":";

        let artifacts = get_valid_artifacts(&self.libraries);
        let mut jars = artifacts
            .into_iter()
            .map(|a| a.path.to_string())
            .collect::<Vec<String>>();
        jars.push(self.get_client_path());

        jars.join(separator)
    }
}

async fn install_client(downloads: &Downloads) -> Result<()> {
    let client_path = VERSIONS_DIR.join(&downloads.client.path);
    download_file(
        &downloads.client.url,
        &client_path,
        Some(&downloads.client.sha1),
    )
    .await?;

    Ok(())
}

pub async fn install(version: &minecraft_version_manifest::Version) -> Result<()> {
    let version_dir = VERSIONS_DIR.join(&version.id);
    fs::create_dir_all(&version_dir).await?;

    let meta_path = version_dir.join("meta.json");
    download_file(&version.url, &meta_path, Some(&version.sha1)).await?;
    let meta = fs::read_to_string(meta_path).await?;
    let meta: MinecraftVersionMeta = serde_json::from_str(&meta)?;

    minecraft_assets::install(&meta.asset_index).await?;
    minecraft_libraries::install(&meta.libraries).await?;
    install_client(&meta.downloads).await?;

    Ok(())
}

pub async fn get(version_id: &str) -> Result<MinecraftVersionMeta> {
    let version_dir = VERSIONS_DIR.join(version_id);
    let meta_path = version_dir.join("meta.json");
    let meta = fs::read_to_string(meta_path).await?;
    let meta: MinecraftVersionMeta = serde_json::from_str(&meta)?;

    Ok(meta)
}
