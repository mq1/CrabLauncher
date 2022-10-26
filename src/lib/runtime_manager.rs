// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

use color_eyre::eyre::Result;
use druid::im::Vector;
use flate2::read::GzDecoder;
use futures_util::StreamExt;
use once_cell::sync::Lazy;
use serde::Deserialize;
use tar::Archive;
use tempfile::tempfile;
use tokio::fs;
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

#[derive(Deserialize, Debug)]
pub struct Package {
    pub link: Url,
    pub size: usize,
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

pub async fn get_assets_info(java_version: &str) -> Result<Assets> {
    let url = format!("{ADOPTIUM_API_ENDPOINT}/v3/assets/latest/{java_version}/hotspot?architecture={ARCH_STRING}&image_type=jre&os={OS_STRING}&vendor=eclipse");

    let mut response = HTTP_CLIENT
        .get(url)
        .send()
        .await?
        .json::<Vec<Assets>>()
        .await?;

    let assets = response.pop().unwrap();

    Ok(assets)
}

pub async fn is_updated(assets: &Assets) -> Result<bool> {
    let dir = format!("{}-jre", assets.release_name);
    let runtime_path = RUNTIMES_DIR
        .join(assets.version.major.to_string())
        .join(dir);

    if !runtime_path.exists() {
        return Ok(false);
    }

    Ok(true)
}

fn extract_archive(file: &File, destination_path: &Path) -> Result<()> {
    if cfg!(target_os = "windows") {
        let mut archive = ZipArchive::new(file)?;
        archive.extract(destination_path)?;
    } else {
        let tar = GzDecoder::new(file);
        let mut archive = Archive::new(tar);
        archive.unpack(destination_path)?;
    }

    Ok(())
}

async fn install(assets: &Assets, event_sink: &druid::ExtEventSink) -> Result<()> {
    event_sink.add_idle_callback(move |data: &mut AppState| {
        data.current_message = "Downloading runtime...".to_string();
        data.current_progress = 0.;
        data.current_view = View::Progress;
    });

    let version_dir = RUNTIMES_DIR.join(assets.version.major.to_string());
    fs::create_dir_all(&version_dir).await?;

    let mut stream = HTTP_CLIENT
        .get(assets.binary.package.link.to_owned())
        .send()
        .await?
        .bytes_stream();

    let mut tmpfile = tempfile()?;
    let mut downloaded_bytes = 0;
    let package_size = assets.binary.package.size as f64;

    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        tmpfile.write_all(&chunk)?;
        downloaded_bytes += chunk.len();

        event_sink.add_idle_callback(move |data: &mut AppState| {
            data.current_progress = downloaded_bytes as f64 / package_size;
        });
    }

    event_sink.add_idle_callback(move |data: &mut AppState| {
        data.current_message = "Extracting runtime...".to_string();
        data.current_view = View::Loading;
    });

    extract_archive(&tmpfile, &version_dir)?;

    Ok(())
}

pub async fn update(assets: &Assets, event_sink: &druid::ExtEventSink) -> Result<()> {
    let runtime_dir = RUNTIMES_DIR.join(assets.version.major.to_string());
    if runtime_dir.exists() {
        fs::remove_dir_all(runtime_dir).await?;
    }

    install(assets, event_sink).await
}

pub async fn get_java_path(java_version: &str) -> Result<PathBuf> {
    let mut entries = fs::read_dir(RUNTIMES_DIR.join(java_version)).await?;
    let runtime_dir = entries.next_entry().await?.unwrap().path();

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
