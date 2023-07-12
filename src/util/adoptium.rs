// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuq01@pm.me>
// SPDX-License-Identifier: GPL-3.0-only

use std::{
    env::consts::{ARCH, OS},
    fs,
    path::PathBuf,
};

use anyhow::{anyhow, bail, Result};
use once_cell::sync::Lazy;
use serde::Deserialize;

use crate::{
    util::{DownloadItem, Hash, HashAlgorithm, AGENT},
    BASE_DIR,
};

static RUNTIMES_DIR: Lazy<PathBuf> = Lazy::new(|| BASE_DIR.join("runtimes"));

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
    let arch = match ARCH {
        "x86_64" => "x64",
        "aarch64" => "aarch64",
        _ => bail!("Unsupported architecture: {}", ARCH),
    };

    let os = match OS {
        "windows" => "windows",
        "linux" => "linux",
        "macos" => "mac",
        _ => bail!("Unsupported OS: {}", OS),
    };

    let url = format!(
        "https://api.adoptium.net/v3/assets/latest/{}/hotspot?architecture={}&image_type=jre&os={}&vendor=eclipse",
        java_version, arch, os
    );

    let assets = &AGENT.get(&url).call()?.into_json::<Vec<Assets>>()?[0];

    let mut download_items = Vec::new();

    let path = RUNTIMES_DIR
        .join(java_version)
        .join(format!("{}-jre", assets.release_name));

    if !path.exists() {
        fs::remove_dir_all(RUNTIMES_DIR.join(java_version))?;

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
    }

    Ok(download_items)
}

pub fn get_path(java_version: &str) -> Result<PathBuf> {
    let dir = RUNTIMES_DIR.join(java_version);
    let runtime_dir = fs::read_dir(&dir)?
        .next()
        .ok_or_else(|| anyhow!("No runtime found for version {}", java_version))??;
    let runtime_dir = runtime_dir.path();

    let runtime_path = match OS {
        "windows" => runtime_dir.join("bin").join("java.exe"),
        "macos" => runtime_dir
            .join("Contents")
            .join("Home")
            .join("bin")
            .join("java"),
        "linux" => runtime_dir.join("bin").join("java"),
        _ => bail!("Unsupported OS: {}", OS),
    };

    if !runtime_path.exists() {
        bail!("No runtime found for version {}", java_version);
    }

    Ok(runtime_path)
}
