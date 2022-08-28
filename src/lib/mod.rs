// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use anyhow::Result;
use isahc::{config::RedirectPolicy, prelude::Configurable, HttpClient, ReadResponseExt};

pub mod accounts;
pub mod instances;
pub mod launcher_config;
pub mod minecraft_news;
pub mod runtime_manager;

pub fn download_file(url: &str, path: &str) -> Result<()> {
    let client = HttpClient::builder()
        .redirect_policy(RedirectPolicy::Limit(10))
        .build()?;

    client.get(url)?.copy_to_file(path)?;

    Ok(())
}
