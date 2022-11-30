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
    pub project_id: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Version {
    pub id: String,
    pub name: String,
    pub featured: bool,
}

impl Hit {
    pub fn fetch_versions(&self) -> Result<Vec<Version>> {
        let resp = HTTP_CLIENT.get(format!("https://api.modrinth.com/v2/project/{}/version", self.project_id)).send()?.json()?;

        Ok(resp)
    }
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
