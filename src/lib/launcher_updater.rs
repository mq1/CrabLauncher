// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use anyhow::Result;
use serde::Deserialize;
use version_compare::Version;

use super::HTTP_CLIENT;

const LATEST_RELEASE_URL: &str = "https://api.github.com/repos/mq1/ice-launcher/releases/latest";
const RELEASES_BASE_URL: &str = "https://github.com/mq1/ice-launcher/releases/tag/";

#[derive(Deserialize)]
struct Release {
    tag_name: String,
}

fn get_latest_release() -> Result<Release> {
    let resp = HTTP_CLIENT.get(LATEST_RELEASE_URL).send()?.json()?;

    Ok(resp)
}

pub fn check_for_updates() -> Result<Option<(String, String)>> {
    let latest_release = get_latest_release()?;
    let latest_release = Version::from(&latest_release.tag_name).unwrap();
    let current_version = Version::from(env!("CARGO_PKG_VERSION")).unwrap();

    if latest_release > current_version {
        let url = format!("{}{}", RELEASES_BASE_URL, latest_release);
        return Ok(Some((latest_release.to_string(), url)));
    }

    Ok(None)
}
