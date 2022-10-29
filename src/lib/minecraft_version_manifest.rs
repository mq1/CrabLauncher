// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use std::{
    io::{self, Seek, SeekFrom},
    path::PathBuf,
};

use color_eyre::{eyre::bail, Result};
use druid::{im::Vector, Data, Lens};
use futures_util::StreamExt;
use serde::Deserialize;
use sha1::{Digest, Sha1};
use tokio::{
    fs::{self, File},
    io::AsyncWriteExt,
};

use crate::{AppState, View};

use super::{
    minecraft_version_meta::{MinecraftVersionMeta, META_DIR},
    HTTP_CLIENT,
};

const VERSION_MANIFEST_URL: &str =
    "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json";

#[derive(Deserialize)]
pub struct MinecraftVersionManifest {
    pub latest: Latest,
    pub versions: Vector<Version>,
}

#[derive(Deserialize)]
pub struct Latest {
    pub release: String,
    pub snapshot: String,
}

#[derive(Deserialize, Clone, Data, PartialEq, Eq, Lens)]
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

    pub async fn get_meta(&self, event_sink: &druid::ExtEventSink) -> Result<MinecraftVersionMeta> {
        event_sink.add_idle_callback(move |data: &mut AppState| {
            data.current_view = View::Loading;
            data.current_message = "Downloading version meta...".to_string();
        });

        let meta_path = self.get_meta_path();

        if meta_path.exists() {
            let mut file = std::fs::File::open(&meta_path)?;
            let mut hasher = Sha1::new();
            let mut buffer = Vec::new();

            io::copy(&mut file, &mut hasher)?;
            file.seek(SeekFrom::Start(0))?;
            io::copy(&mut file, &mut buffer)?;

            let hash = hasher.finalize();
            let hex_hash = base16ct::lower::encode_string(&hash);

            if hex_hash != self.sha1 {
                fs::remove_file(&meta_path).await?;
            } else {
                let meta = serde_json::from_slice(&buffer)?;
                return Ok(meta);
            }
        }

        fs::create_dir_all(meta_path.parent().unwrap()).await?;
        let mut stream = HTTP_CLIENT.get(&self.url).send().await?.bytes_stream();
        let mut file = File::create(&meta_path).await?;
        let mut hasher = Sha1::new();
        let mut buffer = Vec::new();

        while let Some(chunk) = stream.next().await {
            let chunk = chunk?;
            file.write_all(&chunk).await?;
            buffer.write_all(&chunk).await?;
            hasher.update(&chunk);
        }

        let hash = hasher.finalize();
        let hex_hash = base16ct::lower::encode_string(&hash);

        if hex_hash != self.sha1 {
            fs::remove_file(&meta_path).await?;
            bail!("Hash mismatch");
        }

        let meta = serde_json::from_slice(&buffer)?;
        Ok(meta)
    }
}

#[derive(Deserialize, Clone, Data, PartialEq, Eq)]
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

async fn fetch_manifest() -> Result<MinecraftVersionManifest> {
    let manifest = HTTP_CLIENT
        .get(VERSION_MANIFEST_URL)
        .send()
        .await?
        .json()
        .await?;

    Ok(manifest)
}

async fn fetch_versions() -> Result<Vector<Version>> {
    let manifest = fetch_manifest().await?;
    let versions = manifest.versions;

    Ok(versions)
}

pub async fn update_available_versions(event_sink: druid::ExtEventSink) -> Result<()> {
    event_sink.add_idle_callback(move |data: &mut AppState| {
        data.new_instance_state.available_minecraft_versions = Vector::new();
        data.current_message = "Fetching available Minecraft versions...".to_string();
        data.current_view = View::Loading;
    });

    let available_versions = fetch_versions().await?;

    event_sink.add_idle_callback(move |data: &mut AppState| {
        data.new_instance_state.available_minecraft_versions = available_versions;
        data.new_instance_state.selected_version =
            Some(data.new_instance_state.available_minecraft_versions[0].clone());
        data.current_view = View::InstanceVersionSelection;
    });

    Ok(())
}
