// SPDX-FileCopyrightText: 2025 Manuel Quarneti <mq1@ik.me>
// SPDX-License-Identifier: GPL-3.0-only

use anyhow::{Context, Result, anyhow};
use directories;
use std::fs;
use std::path::PathBuf;

use crate::APP_NAME;

pub fn get_data_dir() -> Result<PathBuf> {
    let proj = directories::ProjectDirs::from("it", "mq1", APP_NAME)
        .ok_or(anyhow!("Failed to get project dirs"))?;

    let data_dir = proj.data_dir().to_path_buf();
    fs::create_dir_all(&data_dir).context("Failed to create data dir")?;

    Ok(data_dir)
}
