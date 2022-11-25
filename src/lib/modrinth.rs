// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use anyhow::Result;
use serde::Deserialize;

use super::HTTP_CLIENT;

#[derive(Deserialize)]
pub struct Hit {
    pub title: String,
    pub versions: Vec<String>,
}

pub type Hits = Vec<Hit>;

#[derive(Deserialize)]
struct SearchResults {
    hits: Hits,
}

pub fn fetch_modpacks() -> Result<Hits> {
    let resp = HTTP_CLIENT
        .get("https://api.modrinth.com/v2/search?facets=[[\"project_type:modpack\"]]&limit=20")
        .send()?
        .json::<SearchResults>()?;

    Ok(resp.hits)
}
