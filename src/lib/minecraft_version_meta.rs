// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use std::path::PathBuf;

use color_eyre::Result;
use once_cell::sync::Lazy;
use serde::Deserialize;
use tokio::fs;

use super::{
    minecraft_assets::AssetIndexInfo,
    minecraft_libraries::{Library, LIBRARIES_DIR},
    BASE_DIR,
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
    pub asset_index: AssetIndexInfo,
    pub assets: String,
    pub id: String,
    pub downloads: Downloads,
    pub libraries: Vec<Library>,
    #[serde(rename = "mainClass")]
    pub main_class: String,
}

impl MinecraftVersionMeta {
    pub fn get_client_path(&self) -> PathBuf {
        LIBRARIES_DIR
            .join("com")
            .join("mojang")
            .join("minecraft")
            .join(&self.id)
            .join(format!("minecraft-{}-client", &self.id))
            .with_extension("jar")
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

        jars.push(self.get_client_path().to_string_lossy().to_string());

        jars.join(separator)
    }
}

pub async fn get(version_id: &str) -> Result<MinecraftVersionMeta> {
    let meta_path = META_DIR
        .join("net.minecraft")
        .join(version_id)
        .with_extension("json");

    let meta = fs::read_to_string(meta_path).await?;
    let meta: MinecraftVersionMeta = serde_json::from_str(&meta)?;

    Ok(meta)
}
