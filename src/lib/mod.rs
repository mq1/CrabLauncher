// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use std::{io::{self, Seek}, path::PathBuf, fs::File};

use anyhow::{bail, Result};
use attohttpc::Session;
use directories::ProjectDirs;
use flate2::read::GzDecoder;
use once_cell::sync::Lazy;
use tar::Archive;
use tempfile::tempfile;
use url::Url;
use zip::ZipArchive;

pub mod accounts;
pub mod instances;
pub mod launcher_config;
pub mod launcher_updater;
mod minecraft_assets;
mod minecraft_libraries;
pub mod minecraft_news;
mod minecraft_rules;
pub mod minecraft_version_manifest;
pub mod minecraft_version_meta;
pub mod modrinth;
pub mod msa;
pub mod runtime_manager;

pub static BASE_DIR: Lazy<PathBuf> = Lazy::new(|| {
    ProjectDirs::from("eu", "mq1", "ice-launcher")
        .expect("Could not get project directories")
        .data_dir()
        .to_path_buf()
});

pub static HTTP_CLIENT: Lazy<Session> = Lazy::new(|| {
    let mut sess = Session::new();
    sess.header(
        "User-Agent",
        concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")),
    );

    sess
});

#[derive(Debug, Clone)]
pub enum HashAlgorithm {
    Sha1,
    Sha256,
}

#[derive(Debug, Clone)]
pub struct DownloadItem {
    pub url: Url,
    pub path: PathBuf,
    pub hash: (String, HashAlgorithm),
}

impl DownloadItem {
    pub fn download(&self) -> Result<()> {
        let mut tmp = tempfile()?;
        HTTP_CLIENT.get(self.url.clone()).send()?.write_to(&tmp)?;

        // Verify checksum
        {
            let hex_hash = match self.hash.1 {
                HashAlgorithm::Sha1 => {
                    use sha1::{Digest, Sha1};

                    let mut hasher = Sha1::new();
                    io::copy(&mut tmp, &mut hasher)?;
                    let hash = hasher.finalize();
                    base16ct::lower::encode_string(&hash)
                }
                HashAlgorithm::Sha256 => {
                    use sha2::{Digest, Sha256};

                    let mut hasher = Sha256::new();
                    io::copy(&mut tmp, &mut hasher)?;
                    let hash = hasher.finalize();
                    base16ct::lower::encode_string(&hash)
                }
            };

            if hex_hash != self.hash.0 {
                bail!("Hash mismatch");
            }
        }

        tmp.seek(io::SeekFrom::Start(0))?;

        // If the file is compressed, decompress it
        if self.path.ends_with(".zip") {
            let mut zip = ZipArchive::new(tmp)?;
            zip.extract(&self.path)?;
        } else if self.path.ends_with(".tar.gz") {
            let tar = GzDecoder::new(tmp);
            let mut archive = Archive::new(tar);
            archive.unpack(&self.path)?;
        } else {
            let mut file = File::create(&self.path)?;
            io::copy(&mut tmp, &mut file)?;
        }

        Ok(())
    }
}
