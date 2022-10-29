// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use std::{collections::HashMap, io::{self, SeekFrom, Seek}, path::PathBuf};

use color_eyre::{eyre::bail, Result};
use futures_util::StreamExt;
use once_cell::sync::Lazy;
use serde::Deserialize;
use sha1::{Digest, Sha1};
use tokio::{
    fs::{self, File},
    io::AsyncWriteExt,
};
use url::Url;

use crate::{AppState, View};

use super::{BASE_DIR, HTTP_CLIENT};

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

impl AssetIndexInfo {
    fn get_path(&self) -> PathBuf {
        INDEXES_DIR.join(format!("{}.json", &self.id))
    }

    pub async fn get(&self, event_sink: &druid::ExtEventSink) -> Result<AssetIndex> {
        let path = self.get_path();
        let url = self.url.clone();

        event_sink.add_idle_callback(move |data: &mut AppState| {
            data.current_view = View::Loading;
            data.current_message = "Downloading asset index...".to_string();
        });

        if path.exists() {
            let mut file = std::fs::File::open(&path)?;
            let mut hasher = Sha1::new();
            let mut buffer = Vec::new();

            io::copy(&mut file, &mut hasher)?;
            file.seek(SeekFrom::Start(0))?;
            io::copy(&mut file, &mut buffer)?;

            let hash = hasher.finalize();
            let hex_hash = base16ct::lower::encode_string(&hash);

            if hex_hash != self.sha1 {
                fs::remove_file(&path).await?;
            } else {
                let index = serde_json::from_slice(&buffer)?;
                return Ok(index);
            }
        }

        fs::create_dir_all(path.parent().unwrap()).await?;
        let mut stream = HTTP_CLIENT.get(url).send().await?.bytes_stream();
        let mut file = File::create(&path).await?;
        let mut hasher = Sha1::new();
        let mut buffer = Vec::new();

        while let Some(chunk) = stream.next().await {
            let chunk = chunk?;
            file.write_all(&chunk).await?;
            buffer.write_all(&chunk).await?;
            hasher.update(&chunk);
        }

        let hash = hasher.finalize();
        let hex_hash = base16ct::lower::encode_string(&hash);

        if hex_hash != self.sha1 {
            fs::remove_file(&path).await?;
            bail!("Hash mismatch");
        }

        let index = serde_json::from_slice(&buffer)?;
        Ok(index)
    }
}

#[derive(Deserialize)]
struct Object {
    pub hash: String,
    pub size: usize,
}

impl Object {
    pub fn get_path(&self) -> PathBuf {
        OBJECTS_DIR.join(&self.hash[..2]).join(&self.hash)
    }

    pub fn get_url(&self) -> Result<Url, url::ParseError> {
        ASSETS_DOWNLOAD_ENDPOINT.join(&format!("{}/{}", &self.hash[..2], &self.hash))
    }
}

#[derive(Deserialize)]
pub struct AssetIndex {
    #[serde(rename = "objects")]
    objects: HashMap<String, Object>,
}

impl AssetIndex {
    pub async fn download_objects(
        &self,
        event_sink: &druid::ExtEventSink,
        total_size: usize,
    ) -> Result<()> {
        event_sink.add_idle_callback(move |data: &mut AppState| {
            data.current_view = View::Progress;
            data.current_message = "Downloading assets...".to_string();
            data.current_progress = 0.;
        });

        let mut downloaded_bytes = 0;
        let total_size = total_size as f64;

        for object in self.objects.values() {
            let path = object.get_path();

            if path.exists() {
                let mut file = std::fs::File::open(&path)?;
                let mut hasher = Sha1::new();

                io::copy(&mut file, &mut hasher)?;

                let hash = hasher.finalize();
                let hex_hash = base16ct::lower::encode_string(&hash);

                if hex_hash != object.hash {
                    fs::remove_file(&path).await?;
                } else {
                    downloaded_bytes += object.size;
                    event_sink.add_idle_callback(move |data: &mut AppState| {
                        data.current_progress = downloaded_bytes as f64 / total_size;
                    });
                    continue;
                }
            }

            fs::create_dir_all(path.parent().unwrap()).await?;
            let url = object.get_url()?;
            let mut stream = HTTP_CLIENT.get(url).send().await?.bytes_stream();
            let mut file = File::create(&path).await?;
            let mut hasher = Sha1::new();

            while let Some(chunk) = stream.next().await {
                let chunk = chunk?;
                file.write_all(&chunk).await?;
                hasher.update(&chunk);
                downloaded_bytes += chunk.len();

                event_sink.add_idle_callback(move |data: &mut AppState| {
                    data.current_progress = downloaded_bytes as f64 / total_size;
                });
            }

            let hash = hasher.finalize();
            let hex_hash = base16ct::lower::encode_string(&hash);

            if hex_hash != object.hash {
                fs::remove_file(&path).await?;
                bail!("Hash mismatch");
            }
        }

        Ok(())
    }
}
