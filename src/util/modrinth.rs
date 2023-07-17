// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuq01@pm.me>
// SPDX-License-Identifier: GPL-3.0-only

use std::{fs, io::BufReader, path::Path};

use anyhow::Result;
use serde::Deserialize;

use crate::util::{DownloadItem, Hash, HashAlgorithm, AGENT};

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
    pub sha512: String,
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

pub fn install_version(version: &Version, dest_dir: &Path) -> Result<Vec<DownloadItem>> {
    let tmp_dir = tempfile::tempdir()?;

    let file = &version.files[0];

    let hash = Hash {
        function: HashAlgorithm::Sha512,
        hash: file.hashes.sha512.to_owned(),
    };

    DownloadItem {
        url: file.url.to_owned(),
        path: tmp_dir.path().to_path_buf(),
        hash: Some(hash),
        extract: true,
    }
    .download_file()?;

    let mut items = Vec::new();

    // parse modrinth.index.json
    {
        #[derive(Deserialize)]
        struct File {
            path: String,
            hashes: Hashes,
            downloads: Vec<String>,
        }

        #[derive(Deserialize)]
        struct Dependencies {
            minecraft: String,
            #[serde(rename = "fabric-loader")]
            fabric_loader: Option<String>,
        }

        #[derive(Deserialize)]
        struct Index {
            files: Vec<File>,
            dependencies: Dependencies,
        }

        let index = tmp_dir.path().join("modrinth.index.json");
        let index = BufReader::new(fs::File::open(index)?);
        let index = serde_json::from_reader::<_, Vec<File>>(index)?;

        for file in index {
            let hash = Hash {
                function: HashAlgorithm::Sha512,
                hash: file.hashes.sha512.to_owned(),
            };

            items.push(DownloadItem {
                url: file.downloads[0].to_owned(),
                path: dest_dir.join(file.path),
                hash: Some(hash),
                extract: false,
            });
        }
    }

    // copy overrides to dest_dir
    for r#override in tmp_dir.path().join("overrides").read_dir()? {
        let r#override = r#override?;
        let dest = dest_dir.join(r#override.file_name());
        fs::copy(r#override.path(), dest)?;
    }

    Ok(items)
}
