// SPDX-FileCopyrightText: 2023 Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use std::{
    fmt::{self, Display, Formatter},
    fs::{self, File},
    io::{self, BufReader, BufWriter, Cursor},
    path::Path,
};

use anyhow::{anyhow, Result};
use flate2::bufread::GzDecoder;
use mlua::{ExternalError, ExternalResult, Function, Lua, LuaSerdeExt, Value};
use phf::phf_map;
use serde::{Deserialize, Serialize};
use zip::ZipArchive;

use crate::BASE_DIR;

pub static INSTALLERS: phf::Map<&'static str, &'static str> = phf_map! {
    "vanilla" => include_str!("../../modules/installers/vanilla.lua"),
};

pub static RUNTIMES: phf::Map<&'static str, &'static str> = phf_map! {
    "adoptium" => include_str!("../../modules/runtimes/adoptium.lua"),
};

pub fn get_vm() -> mlua::Result<Lua> {
    let lua = Lua::new();

    // fetch and parse json from uri
    let fetch_json = lua.create_function(|lua, uri: String| {
        let resp = ureq::get(&uri).call().to_lua_err()?;
        let json = resp.into_json::<serde_json::Value>().to_lua_err()?;

        lua.to_value(&json)
    })?;
    lua.globals().set("fetch_json", fetch_json)?;

    // download json from uri and write to file
    let download_json = lua.create_function(|lua, (uri, path): (String, String)| {
        let resp = ureq::get(&uri).call().to_lua_err()?;
        let str = resp.into_string()?;
        let json = serde_json::from_str::<serde_json::Value>(&str).to_lua_err()?;

        // write json to file
        let path = BASE_DIR.join(Path::new(&path));
        fs::write(path, str).to_lua_err()?;

        lua.to_value(&json)
    })?;
    lua.globals().set("download_json", download_json)?;

    // download file from uri
    let download_file = lua.create_function(|_, (uri, path): (String, String)| {
        let resp = ureq::get(&uri).call().to_lua_err()?;

        let path = BASE_DIR.join(Path::new(&path));
        let mut writer = BufWriter::new(File::create(path).to_lua_err()?);
        io::copy(&mut resp.into_reader(), &mut writer).to_lua_err()?;

        Ok(())
    })?;
    lua.globals().set("download_file", download_file)?;

    // download and unpack
    let download_and_unpack = lua.create_function(|_, (uri, path): (String, String)| {
        let resp = ureq::get(&uri).call().to_lua_err()?;
        let path = BASE_DIR.join(Path::new(&path));

        if uri.ends_with(".zip") {
            let size = resp
                .header("Content-Length")
                .unwrap()
                .parse::<usize>()
                .to_lua_err()?;

            let mut reader = resp.into_reader();
            let mut cache = Vec::with_capacity(size);
            reader.read_to_end(&mut cache)?;

            let reader = Cursor::new(cache);
            let mut a = ZipArchive::new(reader).to_lua_err()?;
            a.extract(path).to_lua_err()?;
        } else if uri.ends_with(".tar.gz") {
            let reader = BufReader::new(resp.into_reader());
            let d = GzDecoder::new(reader);
            let mut a = tar::Archive::new(d);
            a.unpack(path).to_lua_err()?;
        } else {
            return Err(anyhow!("unsupported archive format: {}", uri).to_lua_err());
        }

        Ok(())
    })?;
    lua.globals()
        .set("download_and_unpack", download_and_unpack)?;

    // get os
    let get_os = lua.create_function(|_, _: ()| {
        let os = std::env::consts::OS;
        Ok(os)
    })?;
    lua.globals().set("get_os", get_os)?;

    // get arch
    let get_arch = lua.create_function(|_, _: ()| {
        let arch = std::env::consts::ARCH;
        Ok(arch)
    })?;
    lua.globals().set("get_arch", get_arch)?;

    Ok(lua)
}

#[derive(Serialize, Deserialize)]
pub struct InstallerInfo {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "IconSVG")]
    pub icon_svg: String,
}

pub fn get_installer_info(installer: &str) -> mlua::Result<InstallerInfo> {
    let lua = get_vm()?;
    let installer = INSTALLERS.get(installer).unwrap();
    lua.load(*installer).exec()?;

    let info = lua.globals().get::<_, Value>("Info")?;

    lua.from_value(info)
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Version {
    pub id: String,
    url: String,
}

impl Display for Version {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}

pub fn get_versions(installer: &str) -> Result<Vec<Version>> {
    let lua = get_vm()?;
    let installer = INSTALLERS.get(installer).unwrap();
    lua.load(*installer).exec()?;

    let get_versions = lua.globals().get::<_, Function>("GetVersions")?;
    let versions = get_versions.call::<_, Vec<Value>>(())?;
    let versions = versions
        .into_iter()
        .map(|v| lua.from_value::<Version>(v).unwrap())
        .collect();

    Ok(versions)
}

pub fn install_version(installer: &str, version: &Version) -> Result<()> {
    let lua = get_vm()?;
    let installer = INSTALLERS.get(&installer).unwrap();
    lua.load(*installer).exec()?;

    let runtime = lua.globals().get::<_, String>("DefaultRuntime")?;
    let runtime = RUNTIMES.get(&runtime).unwrap();
    lua.load(*runtime).exec()?;

    let install = lua.globals().get::<_, Function>("Install")?;
    install.call::<_, ()>(lua.to_value(version))?;

    Ok(())
}
