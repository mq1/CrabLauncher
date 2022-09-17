// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use std::{
    fs::{self, File},
    path::{Path, PathBuf},
};

use color_eyre::eyre::{bail, Result};
use druid::im::Vector;
use isahc::AsyncReadResponseExt;
use once_cell::sync::Lazy;
use serde::Deserialize;
use url::Url;

#[cfg(target_os = "windows")]
use zip::ZipArchive;

#[cfg(not(target_os = "windows"))]
use tar::Archive;

#[cfg(not(target_os = "windows"))]
use flate2::read::GzDecoder;

use crate::lib::download_file;

use super::BASE_DIR;

const ADOPTIUM_API_ENDPOINT: &str = "https://api.adoptium.net";

static RUNTIMES_DIR: Lazy<PathBuf> = Lazy::new(|| BASE_DIR.join("runtimes"));

#[derive(Deserialize)]
pub struct AvailableReleases {
    pub available_lts_releases: Vec<i32>,
    pub available_releases: Vec<i32>,
    pub most_recent_feature_release: i32,
    pub most_recent_feature_version: i32,
    pub most_recent_lts: i32,
    pub tip_version: i32,
}

#[derive(Deserialize)]
struct Package {
    checksum: String,
    link: Url,
    name: String,
    size: i32,
}

#[derive(Deserialize)]
struct Version {
    semver: String,
}

#[derive(Deserialize)]
struct Binary {
    package: Package,
}

#[derive(Deserialize)]
struct Assets {
    binary: Binary,
    version: Version,
}

pub async fn fetch_available_releases() -> Result<AvailableReleases> {
    let url = format!("{ADOPTIUM_API_ENDPOINT}/v3/info/available_releases");
    let response = isahc::get_async(url).await?.json().await?;

    Ok(response)
}

fn get_architecture_string() -> Result<String> {
    #[cfg(target_arch = "x86")]
    return Ok("x86".to_string());

    #[cfg(target_arch = "x86_64")]
    return Ok("x86_64".to_string());

    #[cfg(target_arch = "aarch64")]
    return Ok("aarch64".to_string());

    #[cfg(not(any(target_arch = "x86", target_arch = "x86_64", target_arch = "aarch64")))]
    bail!("Unsupported architecture");
}

fn get_os_string() -> Result<String> {
    #[cfg(target_os = "windows")]
    return Ok("windows".to_string());

    #[cfg(target_os = "macos")]
    return Ok("mac".to_string());

    #[cfg(target_os = "linux")]
    return Ok("linux".to_string());

    #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
    bail!("Unsupported OS");
}

async fn get_assets_info(java_version: &i32) -> Result<Assets> {
    let url = format!("{endpoint}/v3/assets/latest/{version}/hotspot?architecture={arch}&image_type=jre&os={os}&vendor=eclipse",
        endpoint = ADOPTIUM_API_ENDPOINT,
        version = java_version,
        arch = get_architecture_string()?,
        os = get_os_string()?
    );

    println!("Fetching {url}");

    let mut response: Vec<Assets> = isahc::get_async(url).await?.json().await?;
    let assets = response.pop().unwrap();

    Ok(assets)
}

pub fn list() -> Result<Vector<String>> {
    fs::create_dir_all(RUNTIMES_DIR.as_path())?;

    let mut runtimes = Vector::new();

    for entry in fs::read_dir(RUNTIMES_DIR.as_path())? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            runtimes.push_back(path.file_name().unwrap().to_str().unwrap().to_string());
        }
    }

    Ok(runtimes)
}

#[cfg(target_os = "windows")]
fn extract_archive(archive_path: &Path, destination_path: &Path) -> Result<()> {
    let zip = File::open(archive)?;
    let mut archive = ZipArchive::new(file)?;
    archive.extract(RUNTIMES_DIR.join(assets.version.semver))?;

    Ok(())
}

#[cfg(not(target_os = "windows"))]
fn extract_archive(archive_path: &Path, destination_path: &Path) -> Result<()> {
    let tar_gz = File::open(archive_path)?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    archive.unpack(destination_path)?;

    Ok(())
}

pub async fn install(java_version: &i32) -> Result<()> {
    println!("Installing Java {}", java_version);

    let assets = get_assets_info(java_version).await?;

    let download_url = assets.binary.package.link;
    println!("Downloading {}", download_url);

    #[cfg(target_os = "windows")]
    let extension = "zip";

    #[cfg(not(target_os = "windows"))]
    let extension = "tar.gz";

    let download_path = &RUNTIMES_DIR.join(format!("{}.{}", assets.version.semver, extension));

    download_file(download_url.as_str(), &download_path).await?;
    extract_archive(download_path, RUNTIMES_DIR.as_path())?;
    fs::remove_file(download_path)?;

    println!("Java {} installed", java_version);
    Ok(())
}

pub fn remove(runtime: &str) -> Result<()> {
    println!("Removing {runtime}");

    let runtime_path = RUNTIMES_DIR.join(runtime);
    fs::remove_dir_all(runtime_path)?;

    println!("{runtime} removed");
    Ok(())
}

pub fn get_java_path(java_version: &str) -> Result<PathBuf> {
    let available_runtimes = fs::read_dir(RUNTIMES_DIR.as_path())?;
    let mut runtime: Option<PathBuf> = None;

    for file in available_runtimes {
        let file = file?;

        if file.file_name().to_str().unwrap().contains(java_version) {
            runtime = Some(file.path());
            break;
        }
    }

    if runtime.is_none() {
        bail!("No runtime found");
    }

    let runtime = runtime.unwrap();

    let runtime_path = if cfg!(target_os = "windows") {
        runtime.join("bin").join("java.exe")
    } else if cfg!(target_os = "macos") {
        runtime
            .join("Contents")
            .join("Home")
            .join("bin")
            .join("java")
    } else if cfg!(target_os = "linux") {
        runtime.join("bin").join("java")
    } else {
        panic!("Unsupported operating system");
    };

    Ok(runtime_path)
}
