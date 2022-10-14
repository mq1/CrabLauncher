// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use std::path::{Path, PathBuf};

use color_eyre::eyre::Result;
use directories::ProjectDirs;
use futures_util::StreamExt;
use once_cell::sync::Lazy;
use reqwest::IntoUrl;
use sha1::{Digest, Sha1};
use tokio::{fs::{File, self}, io::AsyncWriteExt};
use url::Url;

pub mod accounts;
pub mod instances;
pub mod launcher_config;
pub mod launcher_updater;
pub mod minecraft_assets;
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

pub async fn download_file<U: IntoUrl>(url: U, path: &Path) -> Result<()> {
    fs::create_dir_all(path.parent().unwrap()).await?;

    let mut stream = HTTP_CLIENT.get(url).send().await?.bytes_stream();
    let mut file = File::create(path).await?;

    while let Some(chunk) = stream.next().await {
        file.write_all(&chunk?).await?;
    }

    Ok(())
}

pub fn check_hash<D: Digest + std::io::Write>(path: &Path, known_hash: &str) -> bool {
    fn check<D: Digest + std::io::Write>(path: &Path, known_hash: &str) -> Result<bool> {
        let mut file = std::fs::File::open(&path)?;
        let mut hasher = D::new();
        std::io::copy(&mut file, &mut hasher)?;
        let hash = hasher.finalize();
        let hex_hash = base16ct::lower::encode_string(&hash);

        Ok(hex_hash == known_hash)
    }

    match check::<D>(path, known_hash) {
        Ok(result) => result,
        Err(err) => false,
    }
}
