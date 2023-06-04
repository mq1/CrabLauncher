// SPDX-FileCopyrightText: 2023 Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use std::{
    fs::{File, self},
    io::{Read, Write},
    path::Path,
};

use anyhow::{bail, Result};
use digest::Digest;

pub mod accounts;
pub mod instances;
pub mod lua;
pub mod settings;
pub mod updater;

const USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

pub fn download_file<D: Digest>(url: &str, path: &Path, hash: Option<String>) -> Result<()> {
    fs::create_dir_all(path.parent().unwrap())?;

    if path.exists() {
        return Ok(());
    }

    let response = ureq::get(url).set("User-Agent", USER_AGENT).call()?;

    let content_length = response
        .header("Content-Length")
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(0);

    let mut buffer = Vec::with_capacity(content_length);
    response.into_reader().read_to_end(&mut buffer)?;

    if let Some(hash) = hash {
        let mut hasher = D::new();
        hasher.update(&buffer);
        let digest = hasher.finalize();
        let digest = base16ct::lower::encode_string(&digest);

        if digest != hash {
            bail!("hash mismatch");
        }
    }

    let mut file = File::create(path)?;
    file.write_all(&buffer)?;

    Ok(())
}
