// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuelquarneti@protonmail.com>
// SPDX-License-Identifier: GPL-3.0-only

use std::{
    collections::HashMap,
    fs::{self, File},
    io::BufReader,
    path::PathBuf,
};

use anyhow::Result;
use serde::Deserialize;

use crate::adoptium;
use crate::paths::{ASSETS_DIR, LIBRARIES_DIR, META_DIR};
use crate::{DownloadItem, DownloadQueue, Hash, HashAlgorithm};

#[cfg(target_os = "windows")]
const OS: &str = "windows";

#[cfg(target_os = "linux")]
const OS: &str = "linux";

#[cfg(target_os = "macos")]
const OS: &str = "osx";

#[cfg(target_os = "windows")]
const SEPARATOR: char = ';';

#[cfg(not(target_os = "windows"))]
const SEPARATOR: char = ':';

#[derive(Deserialize)]
struct VersionManifest {
    versions: Vec<Version>,
}

#[derive(Deserialize)]
pub struct Version {
    id: String,
    url: String,
    sha1: String,
}

pub async fn get_versions() -> Result<Vec<String>> {
    let resp = DownloadItem {
        url: "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json".to_string(),
        path: META_DIR.join("version_manifest_v2.json.new"),
        hash: None,
        extract: false,
    }
    .download_json::<VersionManifest>()?;

    fs::rename(
        META_DIR.join("version_manifest_v2.json.new"),
        META_DIR.join("version_manifest_v2.json"),
    )?;

    let versions = resp
        .versions
        .into_iter()
        .map(|v| v.id)
        .collect::<Vec<String>>();

    Ok(versions)
}

#[derive(Deserialize)]
struct AssetIndexMeta {
    id: String,
    sha1: String,
    url: String,
}

#[derive(Deserialize)]
struct Artifact {
    url: String,
    path: String,
    sha1: String,
}

#[derive(Deserialize)]
struct LibraryDownloads {
    artifact: Artifact,
}

#[derive(Deserialize)]
struct Os {
    name: String,
}

#[derive(Deserialize)]
struct Rule {
    action: String,
    os: Os,
}

#[derive(Deserialize)]
struct Library {
    downloads: LibraryDownloads,
    rules: Option<Vec<Rule>>,
}

impl Library {
    pub fn check(&self) -> bool {
        let mut yes = true;

        if let Some(rules) = &self.rules {
            yes = false;

            for rule in rules {
                if rule.action == "allow" && rule.os.name == OS {
                    yes = true;
                }
            }
        }

        let path = &self.downloads.artifact.path;

        if path.contains("linux") && cfg!(not(target_os = "linux")) {
            yes = false;
        } else if path.contains("windows") && cfg!(not(target_os = "windows")) {
            yes = false;
        } else if path.contains("osx") && cfg!(not(target_os = "macos")) {
            yes = false;
        }

        if path.contains("x86") && cfg!(not(target_arch = "x86_64")) {
            yes = false;
        } else if (path.contains("aarch_64") || path.contains("arm64"))
            && cfg!(not(target_arch = "aarch64"))
        {
            yes = false;
        }

        yes
    }
}

#[derive(Deserialize)]
struct ClientArtifact {
    sha1: String,
    url: String,
}

#[derive(Deserialize)]
struct VersionDownloads {
    client: ClientArtifact,
}

#[derive(Deserialize)]
pub struct VersionMeta {
    id: String,
    #[serde(rename = "assetIndex")]
    asset_index: AssetIndexMeta,
    libraries: Vec<Library>,
    #[serde(rename = "mainClass")]
    pub main_class: String,
    pub assets: String,
    downloads: VersionDownloads,
}

impl VersionMeta {
    pub fn load(id: &str) -> Result<Self> {
        let path = META_DIR.join("versions").join(format!("{}.json", id));
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let version_meta = serde_json::from_reader(reader)?;

        Ok(version_meta)
    }

    fn get_client_path(&self) -> PathBuf {
        LIBRARIES_DIR
            .join("com")
            .join("mojang")
            .join("minecraft")
            .join(self.id.as_str())
            .join(format!("minecraft-{}-client.jar", self.id))
    }

    pub fn get_classpath(&self) -> Result<String> {
        let mut paths = vec![self.get_client_path()];

        for library in &self.libraries {
            if library.check() {
                let path = LIBRARIES_DIR.join(&library.downloads.artifact.path);

                paths.push(path);
            }
        }

        let classpath = paths
            .into_iter()
            .map(|p| p.to_str().unwrap().to_string())
            .collect::<Vec<String>>()
            .join(&SEPARATOR.to_string());

        Ok(classpath)
    }
}

#[derive(Deserialize)]
struct Object {
    hash: String,
}

#[derive(Deserialize)]
struct AssetIndex {
    objects: HashMap<String, Object>,
}

pub fn download_version(id: &str) -> Result<DownloadQueue> {
    let version_manifest = {
        let path = META_DIR.join("version_manifest_v2.json");
        let contents = fs::read_to_string(path)?;
        serde_json::from_str::<VersionManifest>(&contents)?
    };

    let version = version_manifest
        .versions
        .into_iter()
        .find(|v| v.id == id)
        .unwrap();

    // download version meta
    let version_meta = DownloadItem {
        url: version.url,
        path: META_DIR.join("versions").join(format!("{}.json", id)),
        hash: Some(Hash {
            hash: version.sha1,
            function: HashAlgorithm::Sha1,
        }),
        extract: false,
    }
    .download_json::<VersionMeta>()?;

    let mut download_items = vec![];

    // download client
    download_items.push(DownloadItem {
        url: version_meta.downloads.client.url.clone(),
        path: version_meta.get_client_path(),
        hash: Some(Hash {
            hash: version_meta.downloads.client.sha1,
            function: HashAlgorithm::Sha1,
        }),
        extract: false,
    });

    download_items.extend_from_slice(&adoptium::install("17")?);

    let asset_index = DownloadItem {
        url: version_meta.asset_index.url,
        path: ASSETS_DIR
            .join("indexes")
            .join(format!("{}.json", version_meta.asset_index.id)),
        hash: Some(Hash {
            hash: version_meta.asset_index.sha1,
            function: HashAlgorithm::Sha1,
        }),
        extract: false,
    }
    .download_json::<AssetIndex>()?;

    for value in asset_index.objects.into_values() {
        let hash = Hash {
            hash: value.hash,
            function: HashAlgorithm::Sha1,
        };

        let path = ASSETS_DIR.join("objects").join(&hash.get_path());

        download_items.push(DownloadItem {
            url: format!(
                "https://resources.download.minecraft.net/{}",
                hash.get_path()
            ),
            path,
            hash: Some(hash),
            extract: false,
        });
    }

    for library in version_meta.libraries {
        if library.check() {
            let hash = Hash {
                hash: library.downloads.artifact.sha1,
                function: HashAlgorithm::Sha1,
            };

            let path = LIBRARIES_DIR.join(library.downloads.artifact.path);

            download_items.push(DownloadItem {
                url: library.downloads.artifact.url,
                path,
                hash: Some(hash),
                extract: false,
            });
        }
    }

    Ok(DownloadQueue::new(download_items))
}
