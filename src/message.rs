// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuelquarneti@protonmail.com>
// SPDX-License-Identifier: GPL-3.0-only

use crate::pages::Page;
use crate::version_manifest::{Version, VersionManifest};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub enum Message {
    Error(String),
    ChangePage(Page),
    ChangeVanillaInstallerName(String),
    VersionManifestFetched(Result<VersionManifest, Arc<reqwest::Error>>),
    ChangeVanillaInstallerVersion(usize),
}
