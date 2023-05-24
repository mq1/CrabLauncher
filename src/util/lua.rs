// SPDX-FileCopyrightText: 2023 Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use std::{fs, io::BufReader, path::PathBuf};

use anyhow::Result;
use flate2::bufread::GzDecoder;
use mlua::{ExternalResult, Lua, LuaSerdeExt};
use serde_json::Value;
use tar::Archive;

use crate::BASE_DIR;

const MODULES_URL: &str = "https://github.com/mq1/icy-launcher/archive/refs/heads/modules.tar.gz";

pub fn get_vm() -> Result<Lua> {
    let lua = Lua::new();

    let fetch_json = lua.create_function(|lua, uri: String| {
        let resp = ureq::get(&uri).call().to_lua_err()?;
        let json = resp.into_json::<Value>().to_lua_err()?;

        lua.to_value(&json)
    })?;
    lua.globals().set("fetch_json", fetch_json)?;

    Ok(lua)
}

pub fn download_modules() -> Result<()> {
    let dir = BASE_DIR.join("modules");

    let resp = ureq::get(MODULES_URL).call()?;
    let reader = BufReader::new(resp.into_reader());
    let mut archive = Archive::new(GzDecoder::new(reader));

    // remove old modules
    if dir.exists() {
        fs::remove_dir_all(&dir)?;
    }

    archive.unpack(BASE_DIR.as_path())?;

    // rename modules dir
    {
        let old_dir = BASE_DIR.join("icy-launcher-modules");
        fs::rename(old_dir, dir)?;
    }

    Ok(())
}

pub fn list_installers() -> Result<Vec<PathBuf>> {
    let dir = BASE_DIR.join("modules").join("installers");

    let installers = fs::read_dir(dir)?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            let ext = path.extension()?;
            if ext != "lua" {
                return None;
            }

            Some(path)
        })
        .collect();

    Ok(installers)
}
