// SPDX-FileCopyrightText: 2023 Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use std::{
    fs::{self, File},
    io::{self, BufReader, BufWriter, Read, Seek},
    path::{Path, PathBuf},
};

use anyhow::{anyhow, bail, Result};
use digest::Digest;
use flate2::bufread::GzDecoder;
use once_cell::sync::Lazy;
use sha1::Sha1;
use sha2::Sha256;
use tar::Archive;
use tempfile::{tempfile, NamedTempFile};
use ureq::{Agent, AgentBuilder};
use zip::ZipArchive;

pub mod accounts;
pub mod instances;
mod runtime_manager;
pub mod settings;
pub mod updater;
pub mod vanilla_installer;

const USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));
pub static AGENT: Lazy<Agent> = Lazy::new(|| AgentBuilder::new().user_agent(USER_AGENT).build());

pub enum HashAlgorithm {
    Sha1,
    Sha256,
}

pub struct DownloadItem {
    pub url: String,
    pub path: PathBuf,
    pub hash: Option<(String, HashAlgorithm)>,
}

fn calc_hash<D: Digest>(mut reader: impl Read + Seek) -> Result<String> {
    let mut hasher = D::new();

    loop {
        let mut buffer = [0; 1024];
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        hasher.update(&buffer[..count]);
    }

    let digest = hasher.finalize();
    let digest = base16ct::lower::encode_string(&digest);

    Ok(digest)
}

fn check_hash(reader: impl Read + Seek, hash: String, hash_function: String) -> Result<()> {
    println!("checking hash: {hash_function} {hash}");

    let digest = match hash_function.as_str() {
        "sha1" => calc_hash::<Sha1>(reader)?,
        "sha256" => calc_hash::<Sha256>(reader)?,
        _ => bail!("unsupported hash function"),
    };

    if digest != hash {
        bail!("hash mismatch");
    }

    Ok(())
}

pub fn download_file(
    url: &str,
    path: &Path,
    hash: Option<String>,
    hash_function: Option<String>,
) -> Result<()> {
    println!("downloading file: {url} to {}", path.display());

    if path.exists() {
        return Ok(());
    }

    // create parent directory
    {
        let parent = path.parent().ok_or_else(|| anyhow!("invalid path"))?;
        fs::create_dir_all(parent)?;
    }

    let response = AGENT.get(url).call()?;
    let mut file = NamedTempFile::new()?;

    // write to file
    {
        let mut writer = BufWriter::new(&mut file);
        io::copy(&mut response.into_reader(), &mut writer)?;
        writer.seek(io::SeekFrom::Start(0))?;
    }

    // check hash
    if hash.is_some() {
        let mut reader = BufReader::new(&mut file);
        check_hash(&mut reader, hash.unwrap(), hash_function.unwrap())?;
        reader.seek(io::SeekFrom::Start(0))?;
    }

    // move file to destination
    fs::rename(file, path)?;

    Ok(())
}

pub fn download_json(
    url: &str,
    path: &Path,
    hash: Option<String>,
    hash_function: Option<String>,
) -> Result<serde_json::Value> {
    println!("downloading json: {url} to {}", path.display());

    if path.exists() {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let json = serde_json::from_reader(reader)?;

        return Ok(json);
    }

    // create parent directory
    {
        let parent = path.parent().ok_or_else(|| anyhow!("invalid path"))?;
        fs::create_dir_all(parent)?;
    }

    let response = AGENT.get(url).call()?;
    let file = NamedTempFile::new()?;

    // write to file
    {
        let mut writer = BufWriter::new(&file);
        io::copy(&mut response.into_reader(), &mut writer)?;
        writer.seek(io::SeekFrom::Start(0))?;
    }

    // check hash
    if hash.is_some() {
        let mut reader = BufReader::new(&file);
        check_hash(&mut reader, hash.unwrap(), hash_function.unwrap())?;
        reader.seek(io::SeekFrom::Start(0))?;
    }

    let reader = BufReader::new(&file);
    let json = serde_json::from_reader(reader)?;

    // move file to destination
    fs::rename(file, path)?;

    Ok(json)
}

pub fn download_and_unpack(
    url: &str,
    path: &Path,
    hash: Option<String>,
    hash_function: Option<String>,
) -> Result<()> {
    println!("downloading and unpacking: {url} to {}", path.display());

    if path.exists() {
        return Ok(());
    }

    // create parent directory
    {
        let parent = path.parent().ok_or_else(|| anyhow!("invalid path"))?;
        fs::create_dir_all(parent)?;
    }

    let response = AGENT.get(url).call()?;
    let file = tempfile()?;

    // write to file
    {
        let mut writer = BufWriter::new(&file);
        io::copy(&mut response.into_reader(), &mut writer)?;
        writer.seek(io::SeekFrom::Start(0))?;
    }

    // check hash
    if hash.is_some() {
        let mut reader = BufReader::new(&file);
        check_hash(&mut reader, hash.unwrap(), hash_function.unwrap())?;
        reader.seek(io::SeekFrom::Start(0))?;
    }

    // unpack file
    {
        let reader = BufReader::new(&file);

        if url.ends_with(".zip") {
            let mut archive = ZipArchive::new(reader)?;
            archive.extract(path.parent().unwrap())?;
        } else if url.ends_with(".tar.gz") {
            let mut archive = Archive::new(GzDecoder::new(reader));
            archive.unpack(path.parent().unwrap())?;
        } else {
            bail!("unsupported archive format");
        }
    }

    Ok(())
}
