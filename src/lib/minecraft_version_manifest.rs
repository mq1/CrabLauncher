// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use std::{
    fs::{self, File},
    io::{self, BufReader, BufWriter},
    path::PathBuf,
};

use color_eyre::{
    eyre::{bail, eyre},
    Result,
};
use serde::Deserialize;
use sha1::{Digest, Sha1};

use super::{
    minecraft_version_meta::{MinecraftVersionMeta, META_DIR},
    HTTP_CLIENT,
};

const VERSION_MANIFEST_URL: &str =
    "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json";

#[derive(Deserialize)]
pub struct MinecraftVersionManifest {
    pub latest: Latest,
    pub versions: Vec<Version>,
}

#[derive(Deserialize)]
pub struct Latest {
    pub release: String,
    pub snapshot: String,
}

#[derive(Deserialize)]
pub struct Version {
    pub id: String,
    #[serde(rename = "type")]
    version_type: VersionType,
    url: String,
    time: String,
    #[serde(rename = "releaseTime")]
    release_time: String,
    sha1: String,
    #[serde(rename = "complianceLevel")]
    compliance_level: i32,
}

impl Version {
    fn get_meta_path(&self) -> PathBuf {
        META_DIR
            .join("net.minecraft")
            .join(&self.id)
            .with_extension("json")
    }

    fn download_meta(&self) -> Result<()> {
        let path = self.get_meta_path();
        let url = &self.url;

        fs::create_dir_all(path.parent().ok_or(eyre!("Invalid path"))?)?;
        let mut resp = HTTP_CLIENT.get(url).send()?;
        let file = File::create(&path)?;
        let mut writer = BufWriter::new(file);
        io::copy(&mut resp, &mut writer)?;

        Ok(())
    }

    fn check_meta_hash(&self) -> Result<bool> {
        let path = self.get_meta_path();
        let file = File::open(&path)?;
        let mut reader = BufReader::new(file);
        let mut hasher = Sha1::new();
        io::copy(&mut reader, &mut hasher)?;

        let hash = hasher.finalize();
        let hex_hash = base16ct::lower::encode_string(&hash);

        Ok(hex_hash == self.sha1)
    }

    pub fn get_meta(&self) -> Result<MinecraftVersionMeta> {
        let path = self.get_meta_path();

        if path.exists() && !self.check_meta_hash()? {
            fs::remove_file(&path)?;
        }

        if !path.exists() {
            self.download_meta()?;
        }

        if !self.check_meta_hash()? {
            bail!("Asset index hash mismatch");
        }

        let file = File::open(&path)?;
        let reader = BufReader::new(file);
        let meta = serde_json::from_reader(reader)?;

        Ok(meta)
    }
}

#[derive(Deserialize)]
pub enum VersionType {
    #[serde(rename = "old_alpha")]
    OldAlpha,
    #[serde(rename = "old_beta")]
    OldBeta,
    #[serde(rename = "release")]
    Release,
    #[serde(rename = "snapshot")]
    Snapshot,
}

fn fetch_manifest() -> Result<MinecraftVersionManifest> {
    let manifest = HTTP_CLIENT.get(VERSION_MANIFEST_URL).send()?.json()?;

    Ok(manifest)
}

fn fetch_versions() -> Result<Vec<Version>> {
    let manifest = fetch_manifest()?;
    let versions = manifest.versions;

    Ok(versions)
}
