// SPDX-FileCopyrightText: 2023 Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use std::{
    fs::{self, File},
    io::{self, BufWriter},
};

use anyhow::Result;
use mlua::{ExternalResult, Function, Lua, LuaSerdeExt};
use serde::Deserialize;
use serde_json::Value;

use crate::BASE_DIR;

const MODULES_URL: &str = "https://raw.githubusercontent.com/mq1/icy-launcher/modules";

pub fn get_vm() -> Result<Lua> {
    let lua = Lua::new();

    // fetch and parse json from uri
    let fetch_json = lua.create_function(|lua, uri: String| {
        let resp = ureq::get(&uri).call().to_lua_err()?;
        let json = resp.into_json::<Value>().to_lua_err()?;

        lua.to_value(&json)
    })?;
    lua.globals().set("fetch_json", fetch_json)?;

    // download json from uri and write to file
    let download_json = lua.create_function(|lua, (uri, path): (String, String)| {
        let resp = ureq::get(&uri).call().to_lua_err()?;
        let str = resp.into_string().to_lua_err()?;
        let json = serde_json::from_str::<Value>(&str).to_lua_err()?;

        // write json to file
        let path = BASE_DIR.join(path);
        fs::write(path, str).to_lua_err()?;

        lua.to_value(&json)
    })?;
    lua.globals().set("download_json", download_json)?;

    // download file from uri
    let download_file = lua.create_function(|_, (uri, path): (String, String)| {
        let resp = ureq::get(&uri).call().to_lua_err()?;

        let path = BASE_DIR.join(path);
        let mut writer = BufWriter::new(File::create(path).to_lua_err()?);
        io::copy(&mut resp.into_reader(), &mut writer).to_lua_err()?;

        Ok(())
    })?;
    lua.globals().set("download_file", download_file)?;

    Ok(lua)
}

pub type InstallersIndex = Vec<InstallerInfo>;

#[derive(Deserialize, Debug, Clone)]
pub struct InstallerInfo {
    pub name: String,
    pub icon_svg: String,
}

pub fn get_installers() -> Result<InstallersIndex> {
    let url = format!("{MODULES_URL}/installers/index.json");
    let resp = ureq::get(&url).call()?;
    let json = resp.into_json()?;

    Ok(json)
}

impl InstallerInfo {
    pub fn get_versions(&self) -> Result<Vec<String>> {
        let url = format!("{MODULES_URL}/installers/{}.lua", self.name);
        let script = ureq::get(&url).call()?.into_string()?;

        let lua = get_vm()?;
        lua.load(&script).exec()?;

        let get_versions = lua.globals().get::<_, Function>("GetVersions")?;
        let versions = get_versions.call::<_, Vec<String>>(())?;

        Ok(versions)
    }
}
