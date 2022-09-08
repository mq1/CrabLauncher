// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use std::path::{Path, PathBuf};

use color_eyre::eyre::Result;
use directories::ProjectDirs;
use isahc::{config::RedirectPolicy, prelude::Configurable, HttpClient, ReadResponseExt};
use once_cell::sync::Lazy;

pub mod accounts;
pub mod instances;
pub mod launcher_config;
mod minecraft_assets;
pub mod minecraft_news;
pub mod minecraft_version_manifest;
pub mod minecraft_version_meta;
pub mod runtime_manager;
mod minecraft_libraries;
mod minecraft_rules;

pub static BASE_DIR: Lazy<PathBuf> = Lazy::new(|| {
    ProjectDirs::from("eu", "mq1", "ice-launcher")
        .expect("Could not get project directories")
        .data_dir()
        .to_path_buf()
});

pub fn download_file(url: &str, path: &Path) -> Result<()> {
    if path.exists() {
        println!("File already exists, skipping download");
        return Ok(());
    }

    let client = HttpClient::builder()
        .redirect_policy(RedirectPolicy::Limit(10))
        .build()?;

    client.get(url)?.copy_to_file(path)?;

    Ok(())
}
