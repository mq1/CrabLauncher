// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use std::{fs, path::PathBuf};

use color_eyre::Result;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use super::{
    download_file,
    minecraft_rules::{is_rule_list_valid, Rule},
    BASE_DIR,
};

static LIBRARIES_DIR: Lazy<PathBuf> = Lazy::new(|| BASE_DIR.join("libraries"));

#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
const NATIVES_STRING: &str = "natives-linux";

#[cfg(all(target_os = "macos", target_arch = "x86_64"))]
const NATIVES_STRING: &str = "natives-macos";

#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
const NATIVES_STRING: &str = "natives-macos-arm64";

#[cfg(all(target_os = "windows", target_arch = "x86_64"))]
const NATIVES_STRING: &str = "natives-windows";

#[derive(Serialize, Deserialize)]
pub struct Artifact {
    path: String,
    sha1: String,
    size: i32,
    url: String,
}

#[derive(Serialize, Deserialize)]
struct LibraryDownloads {
    artifact: Artifact,
    rules: Option<Vec<Rule>>,
}

#[derive(Serialize, Deserialize)]
pub struct Library {
    downloads: LibraryDownloads,
    name: String,
}

fn is_valid_artifact(artifact: &Artifact) -> bool {
    if artifact.path.contains("natives") && !artifact.path.contains(NATIVES_STRING) {
        return false;
    }

    #[cfg(not(target_arch = "x86_64"))]
    if artifact.path.contains("x86_64") {
        return false;
    }

    #[cfg(not(target_arch = "aarch64"))]
    if artifact.path.contains("aarch_64") {
        return false;
    }

    return true;
}

fn is_valid_library(library: &Library) -> bool {
    if let Some(rules) = &library.downloads.rules {
        return is_rule_list_valid(rules);
    }

    return true;
}

fn get_valid_artifacts(libraries: &Vec<Library>) -> Result<Vec<&Artifact>> {
    let mut valid_artifacts: Vec<&Artifact> = Vec::new();

    for library in libraries {
        if !is_valid_library(&library) {
            continue;
        }

        if is_valid_artifact(&library.downloads.artifact) {
            valid_artifacts.push(&library.downloads.artifact);
        }
    }

    return Ok(valid_artifacts);
}

pub fn install(libraries: &Vec<Library>) -> Result<()> {
    let artifacts = get_valid_artifacts(libraries)?;

    for artifact in artifacts {
        let library_path = LIBRARIES_DIR.join(&artifact.path);
        fs::create_dir_all(library_path.parent().unwrap())?;
        download_file(&artifact.url, &library_path)?;
    }

    Ok(())
}
