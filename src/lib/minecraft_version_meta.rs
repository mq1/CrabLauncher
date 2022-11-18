// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use std::{
    fs::{self, File},
    io::{self, BufReader, BufWriter},
    path::PathBuf,
};

use anyhow::{anyhow, bail, Result};
use once_cell::sync::Lazy;
use serde::Deserialize;
use sha1::{Digest, Sha1};
use url::Url;

use super::{
    minecraft_assets::AssetIndexInfo,
    minecraft_libraries::{Libraries, LibrariesExt, LIBRARIES_DIR},
    BASE_DIR, HTTP_CLIENT,
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
            .join(format!("minecraft-{}-client", &self.id))
            .with_extension("jar")
    }

    fn check_client_hash(&self) -> Result<bool> {
        let path = self.get_client_path();
        let file = File::open(&path)?;
        let mut reader = BufReader::new(file);
        let mut hasher = Sha1::new();
        io::copy(&mut reader, &mut hasher)?;

        let hash = hasher.finalize();
        let hex_hash = base16ct::lower::encode_string(&hash);

        Ok(hex_hash == self.downloads.client.sha1)
    }

    pub fn download_client(&self) -> Result<()> {
        let path = self.get_client_path();
        let url = &self.downloads.client.url;

        if path.exists() && !self.check_client_hash()? {
            fs::remove_file(&path)?;
        }

        if !path.exists() {
            fs::create_dir_all(path.parent().ok_or(anyhow!("Invalid path"))?)?;
            let mut response = HTTP_CLIENT.get(url).send()?;
            let file = File::create(&path)?;
            let mut writer = BufWriter::new(file);
            io::copy(&mut response, &mut writer)?;
        }

        if !self.check_client_hash()? {
            bail!("Hash mismatch");
        }

        Ok(())
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
        .join(version_id)
        .with_extension("json");

    let file = File::open(&path)?;
    let mut reader = BufReader::new(file);
    let meta = serde_json::from_reader(&mut reader)?;

    Ok(meta)
}
