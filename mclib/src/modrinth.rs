// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use anyhow::Result;
use serde::Deserialize;

use super::HTTP_CLIENT;

#[derive(Deserialize, Debug, Clone)]
pub struct Hit {
    pub title: String,
    pub versions: Vec<String>,
    pub latest_version: String,
    pub slug: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct SearchResults {
    pub hits: Vec<Hit>,
}

pub fn fetch_modpacks() -> Result<SearchResults> {
    let resp = HTTP_CLIENT
        .get("https://api.modrinth.com/v2/search?facets=[[\"project_type:modpack\"]]&limit=20")
        .send()?
        .json()?;

    Ok(resp)
}
