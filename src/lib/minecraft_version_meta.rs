// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use std::path::PathBuf;

use color_eyre::Result;
use once_cell::sync::Lazy;
use serde::Deserialize;
use smol::fs;

use super::{
    download_file,
    minecraft_assets::AssetIndex,
    minecraft_libraries::{Library, LIBRARIES_DIR},
    BASE_DIR,
};

pub const VERSIONS_DIR: Lazy<PathBuf> = Lazy::new(|| BASE_DIR.join("versions"));

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
    pub size: u32,
    pub url: String,
}

#[derive(Deserialize)]
pub struct Downloads {
    pub client: Download,
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

        let mut jars = self
            .libraries
            .iter()
            .filter(|l| l.is_valid())
            .map(|l| l.get_path())
            .collect::<Vec<String>>();

        jars.push(self.get_client_path());

        jars.join(separator)
    }

    async fn install_libraries(&self) -> Result<()> {
        for library in &self.libraries {
            if !library.is_valid() {
                continue;
            }

            let library_path = LIBRARIES_DIR.join(&library.downloads.artifact.path);
            fs::create_dir_all(library_path.parent().unwrap()).await?;
            download_file(
                &library.downloads.artifact.url,
                &library_path,
                Some(&library.downloads.artifact.sha1),
            )
            .await?;
        }

        Ok(())
    }

    async fn install_client(&self) -> Result<()> {
        let client_path = VERSIONS_DIR.join(&self.id).join("client.jar");
        download_file(
            &self.downloads.client.url,
            &client_path,
            Some(&self.downloads.client.sha1),
        )
        .await?;

        Ok(())
    }

    pub async fn install(&self) -> Result<()> {
        let assets_result = self.asset_index.install();
        let libraries_result = self.install_libraries();
        let client_result = self.install_client();

        assets_result.await?;
        libraries_result.await?;
        client_result.await?;

        Ok(())
    }
}

pub async fn get(version_id: &str) -> Result<MinecraftVersionMeta> {
    let version_dir = VERSIONS_DIR.join(version_id);
    let meta_path = version_dir.join("meta.json");
    let meta = fs::read_to_string(meta_path).await?;
    let meta: MinecraftVersionMeta = serde_json::from_str(&meta)?;

    Ok(meta)
}
