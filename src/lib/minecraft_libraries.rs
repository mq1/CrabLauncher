// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use std::{io, path::PathBuf};

use async_trait::async_trait;
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
    minecraft_rules::{is_rule_list_valid, Rule},
    BASE_DIR, HTTP_CLIENT,
};

pub static LIBRARIES_DIR: Lazy<PathBuf> = Lazy::new(|| BASE_DIR.join("libraries"));

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
const NATIVES_STRING: &str = "natives-linux";

#[cfg(all(target_os = "macos", target_arch = "x86_64"))]
const NATIVES_STRING: &str = "natives-macos";

#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
const NATIVES_STRING: &str = "natives-macos-arm64";

#[cfg(all(target_os = "windows", target_arch = "x86_64"))]
const NATIVES_STRING: &str = "natives-windows";

#[derive(Deserialize, Clone)]
pub struct Artifact {
    pub path: String,
    pub sha1: String,
    pub size: usize,
    pub url: String,
}

impl Artifact {
    fn is_valid(&self) -> bool {
        if self.path.contains("natives") && !self.path.contains(NATIVES_STRING) {
            return false;
        }

        #[cfg(not(target_arch = "x86_64"))]
        if self.path.contains("x86_64") {
            return false;
        }

        #[cfg(not(target_arch = "aarch64"))]
        if self.path.contains("aarch_64") {
            return false;
        }

        return true;
    }

    pub fn get_path(&self) -> PathBuf {
        LIBRARIES_DIR.join(&self.path)
    }
}

#[derive(Deserialize, Clone)]
pub struct LibraryDownloads {
    pub artifact: Artifact,
    rules: Option<Vec<Rule>>,
}

#[derive(Deserialize, Clone)]
pub struct Library {
    pub downloads: LibraryDownloads,
}

impl Library {
    pub fn is_valid(&self) -> bool {
        if !self.downloads.artifact.is_valid() {
            return false;
        }

        if let Some(rules) = &self.downloads.rules {
            return is_rule_list_valid(rules);
        }

        return true;
    }

    pub fn get_path(&self) -> PathBuf {
        LIBRARIES_DIR.join(&self.downloads.artifact.path)
    }
}

pub type Libraries = Vec<Library>;

#[async_trait]
pub trait LibrariesExt {
    fn get_valid_libraries(&self) -> Libraries;
    async fn download(&self, event_sink: &druid::ExtEventSink) -> Result<()>;
}

#[async_trait]
impl LibrariesExt for Libraries {
    fn get_valid_libraries(&self) -> Libraries {
        self.iter()
            .filter(|library| library.is_valid())
            .cloned()
            .collect()
    }

    async fn download(&self, event_sink: &druid::ExtEventSink) -> Result<()> {
        event_sink.add_idle_callback(move |data: &mut AppState| {
            data.current_view = View::Progress;
            data.current_message = "Downloading libraries...".to_string();
            data.current_progress = 0.;
        });

        let total_bytes =
            self.iter()
                .fold(0, |acc, library| acc + library.downloads.artifact.size) as f64;

        let mut downloaded_bytes = 0;

        for library in self {
            let path = library.get_path();

            if path.exists() {
                let mut file = std::fs::File::open(&path)?;
                let mut hasher = Sha1::new();

                io::copy(&mut file, &mut hasher)?;

                let hash = hasher.finalize();
                let hex_hash = base16ct::lower::encode_string(&hash);

                if hex_hash != library.downloads.artifact.sha1 {
                    fs::remove_file(&path).await?;
                } else {
                    downloaded_bytes += library.downloads.artifact.size;
                    event_sink.add_idle_callback(move |data: &mut AppState| {
                        data.current_progress = downloaded_bytes as f64 / total_bytes;
                    });
                    continue;
                }
            }

            fs::create_dir_all(path.parent().unwrap()).await?;
            let url = &library.downloads.artifact.url;
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

            if hex_hash != library.downloads.artifact.sha1 {
                fs::remove_file(&path).await?;
                bail!("Hash mismatch");
            }
        }

        Ok(())
    }
}
