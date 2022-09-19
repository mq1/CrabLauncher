// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use color_eyre::Result;
use isahc::AsyncReadResponseExt;
use serde::Deserialize;
use version_compare::Version;

const LATEST_RELEASE_URL: &str = "https://api.github.com/repos/mq1/ice-launcher/releases/latest";

async fn get_latest_release() -> Result<String> {
    #[derive(Deserialize)]
    struct Release {
        tag_name: String,
    }

    let response: Release = isahc::get_async(LATEST_RELEASE_URL).await?.json().await?;

    Ok(response.tag_name)
}

pub async fn check_for_updates() -> Result<Option<String>> {
    let latest_release = get_latest_release().await?;
    let latest_release = Version::from(&latest_release).unwrap();
    let current_version = Version::from(env!("CARGO_PKG_VERSION")).unwrap();

    if latest_release > current_version {
        Ok(Some(latest_release.to_string()))
    } else {
        Ok(None)
    }
}
