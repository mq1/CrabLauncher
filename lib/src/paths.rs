// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuelquarneti@protonmail.com>
// SPDX-License-Identifier: GPL-3.0-only

use std::fs;
use std::path::PathBuf;

use directories::ProjectDirs;
use once_cell::sync::Lazy;

pub static BASE_DIR: Lazy<PathBuf> = Lazy::new(|| {
    let dir = ProjectDirs::from("eu", "mq1", "CrabLauncher")
        .unwrap()
        .data_dir()
        .to_path_buf();

    fs::create_dir_all(&dir).unwrap();

    dir
});

pub static META_DIR: Lazy<PathBuf> = Lazy::new(|| {
    let dir = BASE_DIR.join("meta");
    fs::create_dir_all(&dir).unwrap();

    dir
});

pub static ASSETS_DIR: Lazy<PathBuf> = Lazy::new(|| {
    let dir = BASE_DIR.join("../../assets");
    fs::create_dir_all(&dir).unwrap();

    dir
});

pub static LIBRARIES_DIR: Lazy<PathBuf> = Lazy::new(|| {
    let dir = BASE_DIR.join("libraries");
    fs::create_dir_all(&dir).unwrap();

    dir
});

pub static RUNTIMES_DIR: Lazy<PathBuf> = Lazy::new(|| {
    let dir = BASE_DIR.join("runtimes");
    fs::create_dir_all(&dir).unwrap();

    dir
});
pub static SETTINGS_PATH: Lazy<PathBuf> = Lazy::new(|| BASE_DIR.join("settings.toml"));

pub static ACCOUNTS_PATH: Lazy<PathBuf> = Lazy::new(|| BASE_DIR.join("accounts.toml"));
