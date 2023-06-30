// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use std::{fs, path::PathBuf};

use anyhow::Result;
use once_cell::sync::Lazy;
use serde::Deserialize;

use crate::{
    util::{DownloadItem, HashAlgorithm, USER_AGENT},
    BASE_DIR,
};

const ADOPTIUM_API_ENDPOINT: &str = "https://api.adoptium.net";

static RUNTIMES_DIR: Lazy<PathBuf> = Lazy::new(|| BASE_DIR.join("runtimes"));

const ARCH_STRING: &str = std::env::consts::ARCH;

#[cfg(any(target_os = "windows", target_os = "linux"))]
const OS_STRING: &str = std::env::consts::OS;

#[cfg(target_os = "macos")]
const OS_STRING: &str = "mac";

#[derive(Deserialize, Debug)]
struct Package {
    checksum: String,
    link: String,
}

#[derive(Deserialize, Debug)]
struct Binary {
    package: Package,
}

#[derive(Deserialize, Debug)]
struct Version {
    major: i32,
}

#[derive(Deserialize, Debug)]
pub struct Assets {
    binary: Binary,
    release_name: String,
    version: Version,
}

impl Assets {
    pub fn get_path(&self) -> PathBuf {
        RUNTIMES_DIR.join(self.version.major.to_string())
    }

    pub fn get_download_item(&self) -> DownloadItem {
        let url = self.binary.package.link.clone();
        let path = self.get_path();
        let hash = Some((self.binary.package.checksum.clone(), HashAlgorithm::Sha256));

        DownloadItem { url, path, hash }
    }
}

pub fn get_assets_info(java_version: &str) -> Result<Assets> {
    let url = format!("{ADOPTIUM_API_ENDPOINT}/v3/assets/latest/{java_version}/hotspot?architecture={ARCH_STRING}&image_type=jre&os={OS_STRING}&vendor=eclipse");
    let mut response = ureq::get(&url)
        .set("User-Agent", USER_AGENT)
        .call()?
        .into_json::<Vec<Assets>>()?;
    let assets = response.pop().unwrap();

    Ok(assets)
}

pub fn is_updated(assets: &Assets) -> Result<bool> {
    let dir = format!("{}-jre", assets.release_name);
    let runtime_path = RUNTIMES_DIR
        .join(assets.version.major.to_string())
        .join(dir);

    Ok(runtime_path.exists())
}

pub fn get_java_path(java_version: &str) -> Result<PathBuf> {
    let mut entries = fs::read_dir(RUNTIMES_DIR.join(java_version))?.filter(|entry| {
        let entry = entry.as_ref().unwrap();
        entry.file_type().unwrap().is_dir()
    });

    let runtime_dir = entries.next().unwrap()?.path();

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

    Ok(runtime_path)
}
