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
use once_cell::sync::Lazy;
use serde::Deserialize;
use sha1::{Digest, Sha1};

use crate::{AppState, View};

use super::{
    minecraft_rules::{is_rule_list_valid, Rule},
    BASE_DIR, HTTP_CLIENT,
};

pub static LIBRARIES_DIR: Lazy<PathBuf> = Lazy::new(|| BASE_DIR.join("libraries"));

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
const NATIVES_STRING: &str = "natives-linux";

#[cfg(all(target_os = "macos", target_arch = "x86_64"))]
const NATIVES_STRING: &str = "natives-macos";

#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
const NATIVES_STRING: &str = "natives-macos-arm64";

#[cfg(all(target_os = "windows", target_arch = "x86_64"))]
const NATIVES_STRING: &str = "natives-windows";

#[derive(Deserialize, Clone)]
pub struct Artifact {
    pub path: String,
    pub sha1: String,
    pub size: usize,
    pub url: String,
}

impl Artifact {
    fn is_valid(&self) -> bool {
        if self.path.contains("natives") && !self.path.contains(NATIVES_STRING) {
            return false;
        }

        #[cfg(not(target_arch = "x86_64"))]
        if self.path.contains("x86_64") {
            return false;
        }

        #[cfg(not(target_arch = "aarch64"))]
        if self.path.contains("aarch_64") {
            return false;
        }

        return true;
    }

    pub fn get_path(&self) -> PathBuf {
        LIBRARIES_DIR.join(&self.path)
    }
}

#[derive(Deserialize, Clone)]
pub struct LibraryDownloads {
    pub artifact: Artifact,
    rules: Option<Vec<Rule>>,
}

#[derive(Deserialize, Clone)]
pub struct Library {
    pub downloads: LibraryDownloads,
}

impl Library {
    pub fn is_valid(&self) -> bool {
        if !self.downloads.artifact.is_valid() {
            return false;
        }

        if let Some(rules) = &self.downloads.rules {
            return is_rule_list_valid(rules);
        }

        return true;
    }

    pub fn get_path(&self) -> PathBuf {
        LIBRARIES_DIR.join(&self.downloads.artifact.path)
    }

    pub fn download_artifact(&self) -> Result<()> {
        let path = self.get_path();
        let url = &self.downloads.artifact.url;

        fs::create_dir_all(path.parent().ok_or(eyre!("Invalid path"))?)?;
        let resp = HTTP_CLIENT.get(url).call()?;
        let file = File::create(&path)?;
        let mut writer = BufWriter::new(file);
        io::copy(&mut resp.into_reader(), &mut writer)?;

        Ok(())
    }

    fn check_artifact_hash(&self) -> Result<bool> {
        let path = self.get_path();
        let file = File::open(&path)?;
        let mut reader = BufReader::new(file);
        let mut hasher = Sha1::new();
        io::copy(&mut reader, &mut hasher)?;

        let hash = hasher.finalize();
        let hex_hash = base16ct::lower::encode_string(&hash);

        Ok(hex_hash == self.downloads.artifact.sha1)
    }
}

pub type Libraries = Vec<Library>;

pub trait LibrariesExt {
    fn get_valid_libraries(&self) -> Libraries;
    fn download(&self, event_sink: &druid::ExtEventSink) -> Result<()>;
}

impl LibrariesExt for Libraries {
    fn get_valid_libraries(&self) -> Libraries {
        self.iter()
            .filter(|library| library.is_valid())
            .cloned()
            .collect()
    }

    fn download(&self, event_sink: &druid::ExtEventSink) -> Result<()> {
        event_sink.add_idle_callback(move |data: &mut AppState| {
            data.current_view = View::Progress;
            data.current_message = "Downloading libraries...".to_string();
            data.current_progress = 0.;
        });

        let mut downloaded_artifacts = 0.;
        let artifact_count = self.len() as f64;

        for library in self {
            let path = library.get_path();

            if path.exists() && !library.check_artifact_hash()? {
                fs::remove_file(&path)?;
            }

            if !path.exists() {
                library.download_artifact()?;
            }

            if !library.check_artifact_hash()? {
                bail!("Failed to download object");
            }

            downloaded_artifacts += 1.;
            event_sink.add_idle_callback(move |data: &mut AppState| {
                data.current_progress = downloaded_artifacts / artifact_count;
            });
        }

        Ok(())
    }
}
