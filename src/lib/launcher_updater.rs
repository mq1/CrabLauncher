// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use color_eyre::Result;
use serde::Deserialize;
use version_compare::Version;

use crate::AppState;

use super::HTTP_CLIENT;

const LATEST_RELEASE_URL: &str = "https://api.github.com/repos/mq1/ice-launcher/releases/latest";

#[derive(Deserialize)]
struct Release {
    tag_name: String,
}

fn get_latest_release() -> Result<Release> {
    let resp = HTTP_CLIENT.get(LATEST_RELEASE_URL).call()?.into_json()?;

    Ok(resp)
}

pub fn check_for_updates(event_sink: druid::ExtEventSink) -> Result<()> {
    let latest_release = get_latest_release()?;
    let latest_release = Version::from(&latest_release.tag_name).unwrap();
    let current_version = Version::from(env!("CARGO_PKG_VERSION")).unwrap();

    if latest_release > current_version {
        event_sink.add_idle_callback(|data: &mut AppState| {
            data.is_update_available = true;
        })
    }

    Ok(())
}
