// SPDX-FileCopyrightText: 2023 Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use std::{
    fs::{self, File},
    io::{self, BufReader, BufWriter, Read, Seek},
    path::PathBuf,
};

use anyhow::{anyhow, bail, Result};
use digest::Digest;
use flate2::bufread::GzDecoder;
use once_cell::sync::Lazy;
use sha1::Sha1;
use sha2::Sha256;
use strum_macros::Display;
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

#[derive(Debug, Clone, Display)]
pub enum HashAlgorithm {
    Sha1,
    Sha256,
}

#[derive(Debug, Clone)]
pub struct Hash {
    pub hash: String,
    pub function: HashAlgorithm,
}

impl Hash {
    pub fn get_path(&self) -> String {
        format!("{}/{}", self.hash.chars().take(2).collect::<String>(), self.hash)
    }
}

#[derive(Debug, Clone)]
pub struct DownloadItem {
    pub url: String,
    pub path: PathBuf,
    pub hash: Option<Hash>,
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

fn check_hash(reader: impl Read + Seek, hash: &Hash) -> Result<()> {
    println!("checking hash: {} {}", hash.function, hash.hash);

    let digest = match hash.function {
        HashAlgorithm::Sha1 => calc_hash::<Sha1>(reader)?,
        HashAlgorithm::Sha256 => calc_hash::<Sha256>(reader)?,
    };

    if digest != hash.hash {
        bail!("hash mismatch");
    }

    Ok(())
}

pub fn download_file(item: &DownloadItem) -> Result<()> {
    if item.path.exists() {
        println!("file already exists: {}", item.path.display());
        return Ok(());
    }

    println!("downloading file: {} to {}", item.url, item.path.display());

    // create parent directory
    {
        let parent = item.path.parent().ok_or_else(|| anyhow!("invalid path"))?;
        fs::create_dir_all(parent)?;
    }

    let response = AGENT.get(&item.url).call()?;
    let mut file = NamedTempFile::new()?;

    // write to file
    {
        let mut writer = BufWriter::new(&mut file);
        io::copy(&mut response.into_reader(), &mut writer)?;
        writer.seek(io::SeekFrom::Start(0))?;
    }

    // check hash
    if let Some(hash) = &item.hash {
        let mut reader = BufReader::new(&mut file);
        check_hash(&mut reader, hash)?;
        reader.seek(io::SeekFrom::Start(0))?;
    }

    // move file to destination
    fs::rename(file, &item.path)?;

    Ok(())
}

pub fn download_json<T: for<'a> serde::Deserialize<'a>>(item: &DownloadItem) -> Result<T> {
    if item.path.exists() {
        println!("json already exists: {}", item.path.display());

        let file = File::open(&item.path)?;
        let reader = BufReader::new(file);
        let json = serde_json::from_reader(reader)?;

        return Ok(json);
    }

    println!("downloading json: {} to {}", item.url, item.path.display());

    // create parent directory
    {
        let parent = item.path.parent().ok_or_else(|| anyhow!("invalid path"))?;
        fs::create_dir_all(parent)?;
    }

    let response = AGENT.get(&item.url).call()?;
    let file = NamedTempFile::new()?;

    // write to file
    {
        let mut writer = BufWriter::new(&file);
        io::copy(&mut response.into_reader(), &mut writer)?;
        writer.seek(io::SeekFrom::Start(0))?;
    }

    // check hash
    if let Some(hash) = &item.hash {
        let mut reader = BufReader::new(&file);
        check_hash(&mut reader, hash)?;
        reader.seek(io::SeekFrom::Start(0))?;
    }

    let reader = BufReader::new(&file);
    let json = serde_json::from_reader(reader)?;

    // move file to destination
    fs::rename(file, &item.path)?;

    Ok(json)
}

pub fn download_and_unpack(item: &DownloadItem) -> Result<()> {
    if item.path.exists() {
        println!("file already exists: {}", item.path.display());
        return Ok(());
    }

    println!(
        "downloading and unpacking: {} to {}",
        item.url,
        item.path.display()
    );

    // create parent directory
    {
        let parent = item.path.parent().ok_or_else(|| anyhow!("invalid path"))?;
        fs::create_dir_all(parent)?;
    }

    let response = AGENT.get(&item.url).call()?;
    let file = tempfile()?;

    // write to file
    {
        let mut writer = BufWriter::new(&file);
        io::copy(&mut response.into_reader(), &mut writer)?;
        writer.seek(io::SeekFrom::Start(0))?;
    }

    // check hash
    if let Some(hash) = &item.hash {
        let mut reader = BufReader::new(&file);
        check_hash(&mut reader, &hash)?;
        reader.seek(io::SeekFrom::Start(0))?;
    }

    // unpack file
    {
        let reader = BufReader::new(&file);

        if item.url.ends_with(".zip") {
            let mut archive = ZipArchive::new(reader)?;
            archive.extract(item.path.parent().unwrap())?;
        } else if item.url.ends_with(".tar.gz") {
            let mut archive = Archive::new(GzDecoder::new(reader));
            archive.unpack(item.path.parent().unwrap())?;
        } else {
            bail!("unsupported archive format");
        }
    }

    Ok(())
}
