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
use mlua::{ExternalError, ExternalResult, Function, Lua, LuaSerdeExt, Table};
use zip::ZipArchive;

use crate::BASE_DIR;

pub static INSTALLERS: &[&str] = &[include_str!("../../modules/installers/vanilla.lua")];

pub fn get_vm() -> Result<Lua> {
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
        let str = resp.into_string().to_lua_err()?;
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

pub fn get_installers() -> Result<Vec<Lua>> {
    INSTALLERS
        .iter()
        .map(|code| {
            let lua = get_vm()?;
            lua.load(code.to_owned()).exec()?;
            Ok(lua)
        })
        .collect()
}

#[derive(Debug, Clone)]
pub struct Installer {
    pub code: String,
}

impl Installer {
    pub fn get_name(&self) -> Result<String> {
        let lua = get_vm()?;
        lua.load(&self.code).exec()?;

        let name = lua.globals().get::<_, String>("Name")?;

        Ok(name)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Version {
    id: String,
}

impl Display for Version {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}

pub fn get_versions(lua: &Lua) -> Result<Vec<Version>> {
    let get_versions = lua.globals().get::<_, Function>("GetVersions")?;
    let versions = get_versions.call::<_, Vec<Table>>(())?;

    let versions = versions
        .into_iter()
        .map(|table| {
            let id = table.get::<_, String>("id")?;
            Ok(Version { id })
        })
        .collect::<Result<Vec<_>>>()?;

    Ok(versions)
}
