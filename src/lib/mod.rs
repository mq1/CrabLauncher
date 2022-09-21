// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use std::path::{Path, PathBuf};

use color_eyre::eyre::Result;
use const_format::formatcp;
use directories::ProjectDirs;
use isahc::{config::RedirectPolicy, prelude::Configurable, AsyncReadResponseExt, HttpClient};
use once_cell::sync::Lazy;
use sha1::{Digest, Sha1};
use smol::fs::File;

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

pub const USER_AGENT: &str = formatcp!("{}/{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));

pub static BASE_DIR: Lazy<PathBuf> = Lazy::new(|| {
    ProjectDirs::from("eu", "mq1", "ice-launcher")
        .expect("Could not get project directories")
        .data_dir()
        .to_path_buf()
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

    let client = HttpClient::builder()
        .redirect_policy(RedirectPolicy::Limit(10))
        .default_header("User-Agent", USER_AGENT)
        .build()?;

    let mut resp = client.get_async(url).await?;
    let mut file = File::create(path).await?;
    resp.copy_to(&mut file).await?;

    Ok(())
}
