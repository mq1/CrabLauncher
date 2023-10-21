// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuelquarneti@protonmail.com>
// SPDX-License-Identifier: GPL-3.0-only

use std::{
    fs::{self, File},
    io::{self, BufReader, BufWriter, Read, Seek},
    path::PathBuf,
};
use std::sync::Arc;

use anyhow::{anyhow, bail, Result};
use digest::Digest;
use flate2::bufread::GzDecoder;
use once_cell::sync::Lazy;
use sha1::Sha1;
use sha2::{Sha256, Sha512};
use tar::Archive;
use tempfile::NamedTempFile;
use ureq::{Agent, AgentBuilder};
use zip::ZipArchive;

pub mod accounts;
mod adoptium;
mod fabric;
pub mod instances;
pub mod modrinth;
pub mod settings;
pub mod updater;
pub mod vanilla_installer;
pub mod paths;
mod oauth2_client;

const USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));
pub static AGENT: Lazy<Agent> = Lazy::new(|| {
    AgentBuilder::new()
        .user_agent(USER_AGENT)
        .build()
});

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HashAlgorithm {
    Sha1,
    Sha256,
    Sha512,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hash {
    pub hash: String,
    pub function: HashAlgorithm,
}

impl Hash {
    pub fn get_path(&self) -> String {
        format!(
            "{}/{}",
            self.hash.chars().take(2).collect::<String>(),
            self.hash
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DownloadItem {
    pub url: String,
    pub path: PathBuf,
    pub hash: Option<Hash>,
    pub extract: bool,
}

impl DownloadItem {
    pub fn download_file(&self) -> Result<()> {
        if self.path.exists() {
            println!("file already exists: {}", self.path.display());
            return Ok(());
        }

        println!("downloading file: {} to {}", self.url, self.path.display());

        // create parent directory
        {
            let parent = self
                .path
                .parent()
                .ok_or_else(|| anyhow!("invalid path: {}", self.path.display()))?;
            fs::create_dir_all(parent)?;
        }

        let response = AGENT.get(&self.url).call()?;
        let mut file = NamedTempFile::new()?;

        // write to file
        {
            let mut writer = BufWriter::new(&mut file);
            io::copy(&mut response.into_reader(), &mut writer)?;
            writer.seek(io::SeekFrom::Start(0))?;
        }

        // check hash
        if let Some(hash) = &self.hash {
            let mut reader = BufReader::new(&mut file);
            check_hash(&mut reader, hash)?;
            reader.seek(io::SeekFrom::Start(0))?;
        }

        if self.extract {
            println!("extracting archive: {}", self.path.display());

            let reader = BufReader::new(&file);

            if self.url.ends_with(".zip") || self.url.ends_with(".mrpack") {
                let mut archive = ZipArchive::new(reader)?;
                archive.extract(self.path.parent().unwrap())?;
            } else if self.url.ends_with(".tar.gz") {
                let mut archive = Archive::new(GzDecoder::new(reader));
                archive.unpack(self.path.parent().unwrap())?;
            } else {
                fs::remove_file(&self.path)?;
                bail!("unsupported archive format: {}", self.url);
            }

            fs::remove_file(&file)?;
        } else {
            // move file to destination
            fs::rename(file, &self.path)?;
        }

        Ok(())
    }

    pub fn download_json<T: for<'a> serde::Deserialize<'a>>(&self) -> Result<T> {
        if self.path.exists() {
            println!("json already exists: {}", self.path.display());

            let file = File::open(&self.path)?;
            let reader = BufReader::new(file);
            let json = serde_json::from_reader(reader)?;

            return Ok(json);
        }

        println!("downloading json: {} to {}", self.url, self.path.display());

        // create parent directory
        {
            let parent = self
                .path
                .parent()
                .ok_or_else(|| anyhow!("invalid path: {}", self.path.display()))?;
            fs::create_dir_all(parent)?;
        }

        let response = AGENT.get(&self.url).call()?;
        let file = NamedTempFile::new()?;

        // write to file
        {
            let mut writer = BufWriter::new(&file);
            io::copy(&mut response.into_reader(), &mut writer)?;
            writer.seek(io::SeekFrom::Start(0))?;
        }

        // check hash
        if let Some(hash) = &self.hash {
            let mut reader = BufReader::new(&file);
            check_hash(&mut reader, hash)?;
            reader.seek(io::SeekFrom::Start(0))?;
        }

        let reader = BufReader::new(&file);
        let json = serde_json::from_reader(reader)?;

        // move file to destination
        fs::rename(file, &self.path)?;

        Ok(json)
    }
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
    let digest = format!("{:x?}", digest.as_slice());

    Ok(digest)
}

fn check_hash(reader: impl Read + Seek, hash: &Hash) -> Result<()> {
    println!("checking hash: {:?} {}", hash.function, hash.hash);

    let digest = match hash.function {
        HashAlgorithm::Sha1 => calc_hash::<Sha1>(reader)?,
        HashAlgorithm::Sha256 => calc_hash::<Sha256>(reader)?,
        HashAlgorithm::Sha512 => calc_hash::<Sha512>(reader)?,
    };

    if digest != hash.hash {
        bail!("invalid hash: {}", hash.hash);
    }

    Ok(())
}

#[derive(Debug, Clone)]
pub struct DownloadQueue(Vec<DownloadItem>);

impl DownloadQueue {
    pub fn new(items: Vec<DownloadItem>) -> Self {
        Self(items)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn download_next(&mut self) -> Result<bool> {
        if let Some(item) = self.0.pop() {
            item.download_file()?;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
