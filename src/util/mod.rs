// SPDX-FileCopyrightText: 2023 Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use std::{
    fs::{self, File},
    io::{BufReader, BufWriter, Cursor, Read, Write},
    path::Path,
};

use anyhow::{bail, Result};
use digest::DynDigest;
use flate2::read::GzDecoder;
use sha1::Sha1;
use sha2::Sha256;
use tar::Archive;
use zip::ZipArchive;

pub mod accounts;
pub mod instances;
pub mod lua;
pub mod settings;
pub mod updater;

const USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

pub fn get_hasher(name: Option<String>) -> Result<Option<Box<dyn DynDigest>>> {
    if let Some(name) = name {
        match name.as_str() {
            "sha1" => Ok(Some(Box::new(Sha1::default()))),
            "sha256" => Ok(Some(Box::new(Sha256::default()))),
            _ => bail!("unsupported hash function"),
        }
    } else {
        Ok(None)
    }
}

pub fn fetch_file(
    url: &str,
    hash: Option<String>,
    hasher: Option<&mut dyn DynDigest>,
) -> Result<Vec<u8>> {
    let response = ureq::get(url).set("User-Agent", USER_AGENT).call()?;

    let content_length = response
        .header("Content-Length")
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(0);

    let mut buffer = Vec::with_capacity(content_length);
    response.into_reader().read_to_end(&mut buffer)?;

    if let Some(hash) = hash {
        let mut hasher = hasher.unwrap();
        hasher.update(&buffer);
        let digest = hasher.finalize_reset();
        let digest = base16ct::lower::encode_string(&digest);

        if digest != hash {
            bail!("hash mismatch");
        }
    }

    Ok(buffer)
}

pub fn download_file(
    url: &str,
    path: &Path,
    hash: Option<String>,
    hasher: Option<&mut dyn DynDigest>,
) -> Result<()> {
    if path.exists() {
        return Ok(());
    }

    fs::create_dir_all(path.parent().unwrap())?;
    let bytes = fetch_file(url, hash, hasher)?;

    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);
    writer.write_all(&bytes)?;

    Ok(())
}

pub fn download_json(
    url: &str,
    path: &Path,
    hash: Option<String>,
    hasher: Option<&mut dyn DynDigest>,
) -> Result<serde_json::Value> {
    if path.exists() {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let json = serde_json::from_reader(reader)?;

        return Ok(json);
    }

    fs::create_dir_all(path.parent().unwrap())?;
    let bytes = fetch_file(url, hash, hasher)?;

    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);
    writer.write_all(&bytes)?;

    let json = serde_json::from_slice(&bytes)?;
    Ok(json)
}

pub fn download_and_unpack(
    url: &str,
    path: &Path,
    hash: Option<String>,
    hasher: Option<&mut dyn DynDigest>,
) -> Result<()> {
    if path.exists() {
        return Ok(());
    }

    fs::create_dir_all(path.parent().unwrap())?;
    let bytes = fetch_file(url, hash, hasher)?;

    if url.ends_with(".zip") {
        let mut archive = ZipArchive::new(Cursor::new(bytes))?;
        archive.extract(path.parent().unwrap())?;
    } else if url.ends_with(".tar.gz") {
        let mut archive = Archive::new(GzDecoder::new(Cursor::new(bytes)));
        archive.unpack(path.parent().unwrap())?;
    } else {
        bail!("unsupported archive format");
    }

    Ok(())
}
