// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use std::path::{Path, PathBuf};

use color_eyre::eyre::Result;
use directories::ProjectDirs;
use futures_util::StreamExt;
use once_cell::sync::Lazy;
use sha1::{Digest, Sha1};
use tokio::{fs::File, io::AsyncWriteExt};

pub mod accounts;
pub mod instances;
pub mod launcher_config;
pub mod launcher_updater;
mod minecraft_assets;
mod minecraft_libraries;
pub mod minecraft_news;
mod minecraft_rules;
pub mod minecraft_version_manifest;
pub mod minecraft_version_meta;
pub mod msa;
pub mod runtime_manager;

pub static BASE_DIR: Lazy<PathBuf> = Lazy::new(|| {
    ProjectDirs::from("eu", "mq1", "ice-launcher")
        .expect("Could not get project directories")
        .data_dir()
        .to_path_buf()
});

pub static HTTP_CLIENT: Lazy<reqwest::Client> = Lazy::new(|| {
    let user_agent = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

    reqwest::Client::builder()
        .user_agent(user_agent)
        .build()
        .expect("Could not create HTTP client")
});

pub async fn download_file(url: &str, path: &Path, sha1: Option<&str>) -> Result<()> {
    if path.exists() {
        if let Some(sha1) = sha1 {
            let mut file = std::fs::File::open(&path)?;
            let mut hasher = Sha1::new();
            std::io::copy(&mut file, &mut hasher)?;
            let hash = hasher.finalize();
            if format!("{:x}", hash) == sha1 {
                return Ok(());
            }
        } else {
            return Ok(());
        }
    }

    let mut stream = HTTP_CLIENT.get(url).send().await?.bytes_stream();
    let mut file = File::create(path).await?;

    while let Some(chunk) = stream.next().await {
        file.write_all(&chunk?).await?;
    }

    Ok(())
}
