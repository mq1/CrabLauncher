// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use std::path::{Path, PathBuf};

use color_eyre::eyre::{bail, Result};
use druid::im::Vector;
use flate2::read::GzDecoder;
use once_cell::sync::Lazy;
use serde::Deserialize;
use tar::Archive;
use tokio::{
    fs::{self, File},
    io::AsyncWriteExt,
};
use url::Url;
use zip::ZipArchive;

use crate::{AppState, View};

use super::{BASE_DIR, HTTP_CLIENT};

const ADOPTIUM_API_ENDPOINT: &str = "https://api.adoptium.net";

static RUNTIMES_DIR: Lazy<PathBuf> = Lazy::new(|| BASE_DIR.join("runtimes"));

const ARCH_STRING: &str = std::env::consts::ARCH;

#[cfg(any(target_os = "windows", target_os = "linux"))]
const OS_STRING: &str = std::env::consts::OS;

#[cfg(target_os = "macos")]
const OS_STRING: &str = "mac";

#[derive(Deserialize)]
pub struct AvailableReleases {
    pub available_lts_releases: Vector<i32>,
    pub available_releases: Vector<i32>,
    pub most_recent_feature_release: i32,
    pub most_recent_feature_version: i32,
    pub most_recent_lts: i32,
    pub tip_version: i32,
}

#[derive(Deserialize)]
pub struct Package {
    pub link: Url,
    name: String,
    pub size: usize,
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

async fn fetch_available_releases() -> Result<AvailableReleases> {
    let url = format!("{ADOPTIUM_API_ENDPOINT}/v3/info/available_releases");
    let response = HTTP_CLIENT.get(url).send().await?.json().await?;

    Ok(response)
}

async fn get_assets_info(java_version: &i32) -> Result<Assets> {
    let url = format!("{ADOPTIUM_API_ENDPOINT}/v3/assets/latest/{java_version}/hotspot?architecture={ARCH_STRING}&image_type=jre&os={OS_STRING}&vendor=eclipse");

    println!("Fetching {url}");

    let mut response = HTTP_CLIENT
        .get(url)
        .send()
        .await?
        .json::<Vec<Assets>>()
        .await?;

    let assets = response.pop().unwrap();

    Ok(assets)
}

pub async fn is_updated(java_version: &i32) -> Result<bool> {
    let assets = get_assets_info(java_version).await?;
    let dir = format!("{}-jre", assets.release_name);
    let runtime_path = RUNTIMES_DIR.join(dir);

    if !runtime_path.exists() {
        return Ok(false);
    }

    Ok(true)
}

pub async fn list() -> Result<Vector<String>> {
    let mut runtimes = Vector::new();

    fs::create_dir_all(RUNTIMES_DIR.as_path()).await?;
    let mut entries = fs::read_dir(RUNTIMES_DIR.as_path()).await?;

    while let Some(entry) = entries.next_entry().await? {
        if entry.path().is_dir() {
            runtimes.push_back(entry.file_name().to_string_lossy().to_string());
        }
    }

    Ok(runtimes)
}

pub async fn remove_all() -> Result<()> {
    fs::remove_dir_all(RUNTIMES_DIR.as_path()).await?;
    fs::create_dir_all(RUNTIMES_DIR.as_path()).await?;

    Ok(())
}

fn extract_archive(archive_path: &Path, destination_path: &Path) -> Result<()> {
    if cfg!(target_os = "windows") {
        let zip = std::fs::File::open(archive_path)?;
        let mut archive = ZipArchive::new(zip)?;
        archive.extract(destination_path)?;
    } else {
        let tar_gz = std::fs::File::open(archive_path)?;
        let tar = GzDecoder::new(tar_gz);
        let mut archive = Archive::new(tar);
        archive.unpack(destination_path)?;
    }

    Ok(())
}

pub async fn install(java_version: i32, event_sink: druid::ExtEventSink) -> Result<druid::ExtEventSink> {
    event_sink.add_idle_callback(move |data: &mut AppState| {
        data.loading_message = "Downloading runtime...".to_string();
        data.current_progress = 0.;
        data.current_view = View::Progress;
    });

    let assets = get_assets_info(&java_version).await?;
    let download_path = RUNTIMES_DIR.join(&assets.binary.package.name);

    let mut resp = HTTP_CLIENT.get(assets.binary.package.link).send().await?;
    let mut file = File::create(&download_path).await?;
    let mut downloaded_bytes = 0;

    while let Some(chunk) = resp.chunk().await? {
        file.write_all(&chunk).await?;
        downloaded_bytes += chunk.len();

        event_sink.add_idle_callback(move |data: &mut AppState| {
            data.current_progress = downloaded_bytes as f64 / assets.binary.package.size as f64;
        });
    }

    event_sink.add_idle_callback(move |data: &mut AppState| {
        data.loading_message = "Extracting runtime...".to_string();
        data.current_view = View::Loading;
    });

    extract_archive(&download_path, RUNTIMES_DIR.as_path())?;
    fs::remove_file(download_path).await?;

    Ok(event_sink)
}

pub async fn remove(runtime: String) -> Result<()> {
    println!("Removing {runtime}");

    let runtime_path = RUNTIMES_DIR.join(&runtime);
    fs::remove_dir_all(runtime_path).await?;

    println!("{runtime} removed");
    Ok(())
}

pub async fn get_java_path(java_version: &u32) -> Result<PathBuf> {
    let java_version = java_version.to_string();

    let mut runtime: Option<PathBuf> = None;

    let mut entries = fs::read_dir(RUNTIMES_DIR.as_path()).await?;
    while let Some(entry) = entries.next_entry().await? {
        if entry.file_name().to_string_lossy().contains(&java_version) {
            runtime = Some(entry.path());
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
    } else {
        runtime.join("bin").join("java")
    };

    Ok(runtime_path)
}
