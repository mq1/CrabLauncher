// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuq01@pm.me>
// SPDX-License-Identifier: GPL-3.0-only

use serde::Deserialize;
use version_compare::Version;

use crate::{types::generic_error::GenericError, util::AGENT};

const LATEST_RELEASE_URL: &str = "https://api.github.com/repos/mq1/icy-launcher/releases/latest";
const RELEASES_BASE_URL: &str = "https://github.com/mq1/icy-launcher/releases/tag/";

#[derive(Deserialize)]
struct Release {
    tag_name: String,
}

async fn get_latest_release() -> Result<Release, GenericError> {
    let resp = AGENT.get(LATEST_RELEASE_URL).call()?.into_json()?;

    Ok(resp)
}

pub async fn check_for_updates() -> Result<Option<(String, String)>, GenericError> {
    let latest_release = get_latest_release().await?;
    let latest_release = Version::from(&latest_release.tag_name).unwrap();
    let current_version = Version::from(env!("CARGO_PKG_VERSION")).unwrap();

    if latest_release > current_version {
        let url = format!("{}{}", RELEASES_BASE_URL, latest_release);
        return Ok(Some((latest_release.to_string(), url)));
    }

    Ok(None)
}
