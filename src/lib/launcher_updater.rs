// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use color_eyre::Result;
use serde::Deserialize;
use version_compare::Version;

use super::USER_AGENT;

const LATEST_RELEASE_URL: &str = "https://api.github.com/repos/mq1/ice-launcher/releases/latest";

#[derive(Deserialize)]
struct Release {
    tag_name: String,
}

async fn get_latest_release() -> Result<Release> {
    let client = reqwest::Client::builder().user_agent(USER_AGENT).build()?;
    let resp = client.get(LATEST_RELEASE_URL).send().await?.json().await?;

    Ok(resp)
}

pub async fn check_for_updates() -> Result<Option<String>> {
    let latest_release = get_latest_release().await?;
    let latest_release = Version::from(&latest_release.tag_name).unwrap();
    let current_version = Version::from(env!("CARGO_PKG_VERSION")).unwrap();

    if latest_release > current_version {
        Ok(Some(latest_release.to_string()))
    } else {
        Ok(None)
    }
}
