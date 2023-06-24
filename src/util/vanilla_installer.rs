// SPDX-FileCopyrightText: 2023 Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use std::fmt::Display;

use anyhow::Result;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Version {
    pub id: String,
    pub url: String,
    pub sha1: String,
}

impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

pub async fn get_versions() -> Result<Vec<Version>> {
    #[derive(Deserialize)]
    struct Response {
        versions: Vec<Version>,
    }

    let resp = ureq::get("https://piston-meta.mojang.com/mc/game/version_manifest_v2.json")
        .call()?
        .into_json::<Response>()?;

    Ok(resp.versions)
}
