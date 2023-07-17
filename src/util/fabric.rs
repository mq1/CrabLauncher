// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuq01@pm.me>
// SPDX-License-Identifier: GPL-3.0-only

use anyhow::Result;

use crate::util::{DownloadItem, AGENT};

pub fn install_version(minecraft_version: &str, fabric_version: &str) -> Result<Vec<DownloadItem>> {
    Ok(vec![])
}
