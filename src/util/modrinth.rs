// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuq01@pm.me>
// SPDX-License-Identifier: GPL-3.0-only

use std::{path::Path, fs};

use anyhow::Result;
use serde::Deserialize;

use crate::util::{AGENT, download_file, DownloadItem, Hash, HashAlgorithm};

#[derive(Deserialize)]
pub struct Project {
    pub project_id: String,
    pub title: String,
    pub icon_url: String,
}

#[derive(Deserialize)]
pub struct Projects {
    pub hits: Vec<Project>,
}

pub fn search_modpacks(query: &str) -> Result<Projects> {
    let url = format!(
        "https://api.modrinth.com/v2/search?query={query}&facets=[[\"project_type:modpack\"]]",
    );

    let resp = AGENT.get(&url).call()?.into_json()?;

    Ok(resp)
}

#[derive(Deserialize)]
pub struct Hashes {
    pub sha1: String,
    pub sha512: String, // todo: use this
}

#[derive(Deserialize)]
pub struct File {
    pub hashes: Hashes,
    pub url: String,
    pub filename: String,
}

#[derive(Deserialize)]
pub struct Version {
    pub name: String,
    pub files: Vec<File>,
}

pub fn get_versions(modpack_id: &str) -> Result<Vec<Version>> {
    let url = format!("https://api.modrinth.com/v2/project/{modpack_id}/version");

    let resp = AGENT.get(&url).call()?.into_json()?;

    Ok(resp)
}

pub fn install_version(version: &Version, dest_dir: &Path) -> Result<()> {
    let tmp_dir = tempfile::tempdir()?;

    let file = &version.files[0];

    let hash = Hash {
        function: HashAlgorithm::Sha1,
        hash: file.hashes.sha1.to_owned(),
    };

    download_file(&DownloadItem {
        url: file.url.to_owned(),
        path: tmp_dir.path().to_path_buf(),
        hash: Some(hash),
        extract: true,
    })?;

    // TODO: parse modrinth.index.json

    // copy overrides to dest_dir
    for r#override in tmp_dir.into_path().join("overrides").read_dir()? {
        let r#override = r#override?;
        let dest = dest_dir.join(r#override.file_name());
        fs::copy(r#override.path(), dest)?;
    }

    Ok(())
}
