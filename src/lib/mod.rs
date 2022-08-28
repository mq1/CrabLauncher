// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use std::{fs, path::PathBuf};

use anyhow::Result;
use directories::ProjectDirs;
use isahc::{config::RedirectPolicy, prelude::Configurable, HttpClient, ReadResponseExt};
use once_cell::sync::Lazy;

pub mod accounts;
pub mod instances;
pub mod launcher_config;
pub mod minecraft_news;
pub mod runtime_manager;

static BASE_DIR: Lazy<PathBuf> = Lazy::new(|| {
    let project_dirs =
        ProjectDirs::from("eu", "mq1", "ice-launcher").expect("Could not get project directories");
    let base_dir = project_dirs.data_dir().to_owned();
    fs::create_dir_all(&base_dir).expect("Could not create base directory");

    base_dir
});

pub fn download_file(url: &str, path: &str) -> Result<()> {
    let client = HttpClient::builder()
        .redirect_policy(RedirectPolicy::Limit(10))
        .build()?;

    client.get(url)?.copy_to_file(path)?;

    Ok(())
}
