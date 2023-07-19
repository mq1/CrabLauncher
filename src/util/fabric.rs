// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuq01@pm.me>
// SPDX-License-Identifier: GPL-3.0-only

use std::path::PathBuf;

use serde::Deserialize;

use crate::{types::generic_error::GenericError, util::{AGENT, DownloadItem, paths::LIBRARIES_DIR}};

#[derive(Deserialize)]
struct FabricLibrary {
    name: String,
    url: String,
}

impl FabricLibrary {
    fn get_path(&self) -> String {
        let components = self.name.split(':').collect::<Vec<_>>();
        let parent_dir = components[0].replace('.', "/");
        let artifact = components[1];
        let version = components[2];

        format!(
            "{}/{}/{}/{}-{}.jar",
            parent_dir,
            artifact,
            version,
            artifact,
            version
        )
    }

    pub fn get_download_url(&self) -> String {
        format!(
            "{}{}",
            self.url,
            self.get_path()
        )
    }

    pub fn get_full_path(&self) -> PathBuf {
        LIBRARIES_DIR.join(self.get_path())
    }
}

#[derive(Deserialize)]
struct FabricMeta {
    libraries: Vec<FabricLibrary>,
    #[serde(rename = "mainClass")]
    main_class: String,
}

pub fn download_version(
    minecraft_version: &str,
    fabric_version: &str,
) -> Result<Vec<DownloadItem>, GenericError> {
    let url = format!(
        "https://meta.fabricmc.net/v2/versions/loader/{}/{}/profile/json",
        minecraft_version,
        fabric_version
    );

    let meta = AGENT
        .get(&url)
        .call()?
        .into_json::<FabricMeta>()?;

    meta.libraries
        .into_iter()
        .map(|lib| {
            Ok(DownloadItem {
                url: lib.get_download_url(),
                path: lib.get_full_path(),
                hash: None,
                extract: false,
            })
        })
        .collect()
}
