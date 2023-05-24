// SPDX-FileCopyrightText: 2023 Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use std::{fs, io::BufReader, path::PathBuf};

use anyhow::Result;
use flate2::bufread::GzDecoder;
use mlua::{ExternalResult, Function, Lua, LuaSerdeExt};
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

    // todo: properly update modules
    if dir.exists() {
        return Ok(());
    }

    let resp = ureq::get(MODULES_URL).call()?;
    let reader = BufReader::new(resp.into_reader());
    let mut archive = Archive::new(GzDecoder::new(reader));

    archive.unpack(BASE_DIR.as_path())?;

    // rename modules dir
    fs::rename(BASE_DIR.join("icy-launcher-modules"), dir)?;

    Ok(())
}

#[derive(Debug, Clone)]
pub struct Installer {
    pub path: PathBuf,
    pub name: String,
    pub icon_svg: Vec<u8>,
}

impl Installer {
    pub fn get_versions(&self) -> Result<Vec<String>> {
        let lua = get_vm()?;
        let str = fs::read_to_string(self.path.as_path())?;
        lua.load(&str).exec()?;

        let get_versions = lua.globals().get::<_, Function>("GetVersions")?;
        let versions = get_versions.call::<_, Vec<String>>(())?;

        Ok(versions)
    }
}

pub fn list_installers() -> Result<Vec<Installer>> {
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
        .collect::<Vec<_>>();

    let lua = get_vm()?;
    let installers = installers
        .into_iter()
        .filter_map(|path| {
            let str = fs::read_to_string(&path).ok()?;
            lua.load(&str).exec().ok()?;

            let name = lua.globals().get::<_, String>("Name").ok()?;
            let icon_svg = lua.globals().get::<_, String>("IconSVG").ok()?;
            let icon_bytes = icon_svg.as_bytes().to_vec();

            Some(Installer {
                path,
                name,
                icon_svg: icon_bytes,
            })
        })
        .collect::<Vec<_>>();

    Ok(installers)
}
