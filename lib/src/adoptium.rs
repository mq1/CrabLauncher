// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuelquarneti@protonmail.com>
// SPDX-License-Identifier: GPL-3.0-only

use std::{fs, path::PathBuf};

use anyhow::{anyhow, bail, Result};
use serde::Deserialize;

use crate::paths::RUNTIMES_DIR;
use crate::{DownloadItem, Hash, HashAlgorithm, AGENT};

#[cfg(target_os = "windows")]
const OS: &str = "windows";

#[cfg(target_os = "linux")]
const OS: &str = "linux";

#[cfg(target_os = "macos")]
const OS: &str = "mac";

#[cfg(target_arch = "x86_64")]
const ARCH: &str = "x64";

#[cfg(target_arch = "aarch64")]
const ARCH: &str = "aarch64";

#[derive(Deserialize)]
struct Package {
    checksum: String,
    link: String,
}

#[derive(Deserialize)]
struct Binary {
    package: Package,
}

#[derive(Deserialize)]
struct Assets {
    binary: Binary,
    release_name: String,
}

pub fn install(java_version: &str) -> Result<Vec<DownloadItem>> {
    let url = format!(
        "https://api.adoptium.net/v3/assets/latest/{}/hotspot?architecture={}&image_type=jre&os={}&vendor=eclipse",
        java_version, ARCH, OS
    );

    let assets = &AGENT.get(&url).call()?.into_json::<Vec<Assets>>()?[0];

    let mut download_items = Vec::new();

    let path = RUNTIMES_DIR
        .join(java_version)
        .join(format!("{}-jre", assets.release_name));

    if !path.exists() {
        let _ = fs::remove_dir_all(RUNTIMES_DIR.join(java_version));

        let url = assets.binary.package.link.to_owned();
        let hash = Some(Hash {
            hash: assets.binary.package.checksum.to_owned(),
            function: HashAlgorithm::Sha256,
        });
        download_items.push(DownloadItem {
            url,
            path,
            hash,
            extract: true,
        });
    } else {
        println!("Runtime already up to date");
    }

    Ok(download_items)
}

pub fn get_path(java_version: &str) -> Result<PathBuf> {
    let dir = RUNTIMES_DIR.join(java_version);
    let runtime_dir = fs::read_dir(&dir)?
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().unwrap().is_dir())
        .next()
        .ok_or_else(|| anyhow!("No runtime found for version {}", java_version))?;
    let runtime_dir = runtime_dir.path();

    let runtime_path = if cfg!(target_os = "windows") {
        runtime_dir.join("bin").join("java.exe")
    } else if cfg!(target_os = "macos") {
        runtime_dir
            .join("Contents")
            .join("Home")
            .join("bin")
            .join("java")
    } else {
        runtime_dir.join("bin").join("java")
    };

    if !runtime_path.exists() {
        bail!("No runtime found for version {}", java_version);
    }

    Ok(runtime_path)
}
