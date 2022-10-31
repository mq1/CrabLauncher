// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use std::{
    fs::{self, File},
    io::{self, BufReader, BufWriter},
    path::{Path, PathBuf},
};

use color_eyre::eyre::{bail, Result};
use flate2::read::GzDecoder;
use once_cell::sync::Lazy;
use serde::Deserialize;
use sha2::{Digest, Sha256};
use tar::Archive;
use tempfile::tempfile;
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

#[derive(Deserialize, Debug)]
struct Package {
    checksum: String,
    link: Url,
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

pub fn get_assets_info(java_version: &str) -> Result<Assets> {
    let url = &format!("{ADOPTIUM_API_ENDPOINT}/v3/assets/latest/{java_version}/hotspot?architecture={ARCH_STRING}&image_type=jre&os={OS_STRING}&vendor=eclipse");

    let mut response = HTTP_CLIENT.get(url).send()?.json::<Vec<Assets>>()?;

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

fn extract_archive(file: &File, destination_path: &Path) -> Result<()> {
    let reader = BufReader::new(file);

    if cfg!(target_os = "windows") {
        let mut archive = ZipArchive::new(reader)?;
        archive.extract(destination_path)?;
    } else {
        let tar = GzDecoder::new(reader);
        let mut archive = Archive::new(tar);
        archive.unpack(destination_path)?;
    }

    Ok(())
}

fn install(assets: &Assets, event_sink: &druid::ExtEventSink) -> Result<()> {
    event_sink.add_idle_callback(move |data: &mut AppState| {
        data.current_message = "Downloading runtime...".to_string();
        data.current_view = View::Loading;
    });

    let version_dir = RUNTIMES_DIR.join(assets.version.major.to_string());
    fs::create_dir_all(&version_dir)?;

    let url = &assets.binary.package.link.clone().to_string();
    let mut resp = HTTP_CLIENT.get(url).send()?;
    let tmpfile = tempfile()?;

    {
        let mut writer = BufWriter::new(&tmpfile);
        io::copy(&mut resp, &mut writer)?;
    }

    {
        let mut reader = BufReader::new(&tmpfile);
        let mut hasher = Sha256::new();
        io::copy(&mut reader, &mut hasher)?;

        let hash = hasher.finalize();
        let hex_hash = base16ct::lower::encode_string(&hash);

        if hex_hash != assets.binary.package.checksum {
            bail!("Hash mismatch");
        }
    }

    event_sink.add_idle_callback(move |data: &mut AppState| {
        data.current_message = "Extracting runtime...".to_string();
        data.current_view = View::Loading;
    });

    extract_archive(&tmpfile, &version_dir)
}

pub fn update(assets: &Assets, event_sink: &druid::ExtEventSink) -> Result<()> {
    let runtime_dir = RUNTIMES_DIR.join(assets.version.major.to_string());
    if runtime_dir.exists() {
        fs::remove_dir_all(runtime_dir)?;
    }

    install(assets, event_sink)
}

pub fn get_java_path(java_version: &str) -> Result<PathBuf> {
    let mut entries = fs::read_dir(RUNTIMES_DIR.join(java_version))?;
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
