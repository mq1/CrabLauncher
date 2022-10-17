// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use std::path::PathBuf;

use color_eyre::Result;
use druid::{im::Vector, Data, Lens};
use serde::Deserialize;

use crate::{AppState, View};

use super::{minecraft_version_meta::META_DIR, HTTP_CLIENT};

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
    pub version_type: VersionType,
    pub url: String,
    pub time: String,
    #[serde(rename = "releaseTime")]
    pub release_time: String,
    pub sha1: String,
    #[serde(rename = "complianceLevel")]
    pub compliance_level: i32,
}

impl Version {
    pub fn get_meta_path(&self) -> PathBuf {
        META_DIR
            .join("net.minecraft")
            .join(&self.id)
            .with_extension("json")
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
        data.loading_message = "Fetching available Minecraft versions...".to_string();
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
