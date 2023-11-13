// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuelquarneti@protonmail.com>
// SPDX-License-Identifier: GPL-3.0-only

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use anyhow::Error;
use std::fs;
use std::path::PathBuf;

use directories::ProjectDirs;
use iced::{Application, Settings};
use once_cell::sync::Lazy;

use crate::launcher::Launcher;
use crate::message::Message;

mod icon;
mod info;
mod instances;
mod launcher;
mod message;
mod navbar;
mod pages;
mod settings;
mod style;
mod vanilla_installer;
mod version_manifest;

pub static BASE_DIR: Lazy<PathBuf> = Lazy::new(|| {
    let dir = ProjectDirs::from("eu", "mq1", "CrabLauncher")
        .unwrap()
        .data_dir()
        .to_path_buf();

    fs::create_dir_all(&dir).unwrap();

    dir
});

pub fn show_error(err: &Error) {
    rfd::MessageDialog::new()
        .set_level(rfd::MessageLevel::Error)
        .set_title("Error")
        .set_description(&err.to_string())
        .show();
}

#[tokio::main]
async fn main() -> iced::Result {
    Launcher::run(Settings::default())
}
