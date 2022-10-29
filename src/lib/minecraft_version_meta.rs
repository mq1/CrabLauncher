// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use std::path::PathBuf;

use color_eyre::{eyre::bail, Result};
use futures_util::StreamExt;
use once_cell::sync::Lazy;
use serde::Deserialize;
use sha1::{Digest, Sha1};
use tokio::{
    fs::{self, File},
    io::AsyncWriteExt,
};

use crate::{AppState, View};

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

    pub async fn download_client(&self, event_sink: &druid::ExtEventSink) -> Result<()> {
        event_sink.add_idle_callback(move |data: &mut AppState| {
            data.current_view = View::Progress;
            data.current_message = "Downloading client...".to_string();
            data.current_progress = 0.;
        });

        let path = self.get_client_path();
        let mut downloaded_bytes = 0;
        let total_bytes = self.downloads.client.size as f64;

        if path.exists() {
            let mut file = std::fs::File::open(&path)?;
            let mut hasher = Sha1::new();

            std::io::copy(&mut file, &mut hasher)?;

            let hash = hasher.finalize();
            let hex_hash = base16ct::lower::encode_string(&hash);

            if hex_hash != self.downloads.client.sha1 {
                fs::remove_file(&path).await?;
            } else {
                downloaded_bytes += self.downloads.client.size;
                event_sink.add_idle_callback(move |data: &mut AppState| {
                    data.current_progress = downloaded_bytes as f64 / total_bytes;
                });
                return Ok(());
            }
        }

        fs::create_dir_all(path.parent().unwrap()).await?;
        let url = &self.downloads.client.url;
        let mut stream = HTTP_CLIENT.get(url).send().await?.bytes_stream();
        let mut file = File::create(&path).await?;
        let mut hasher = Sha1::new();

        while let Some(chunk) = stream.next().await {
            let chunk = chunk?;
            file.write_all(&chunk).await?;
            hasher.update(&chunk);
            downloaded_bytes += chunk.len();

            event_sink.add_idle_callback(move |data: &mut AppState| {
                data.current_progress = downloaded_bytes as f64 / total_bytes;
            });
        }

        let hash = hasher.finalize();
        let hex_hash = base16ct::lower::encode_string(&hash);

        if hex_hash != self.downloads.client.sha1 {
            fs::remove_file(&path).await?;
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

pub async fn get(version_id: &str) -> Result<MinecraftVersionMeta> {
    let meta_path = META_DIR
        .join("net.minecraft")
        .join(version_id)
        .with_extension("json");

    let meta = fs::read_to_string(meta_path).await?;
    let meta: MinecraftVersionMeta = serde_json::from_str(&meta)?;

    Ok(meta)
}
