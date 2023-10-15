// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuelquarneti@protonmail.com>
// SPDX-License-Identifier: GPL-2.0-only

// hide console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use std::path::PathBuf;

use directories::ProjectDirs;
use once_cell::sync::Lazy;

use crate::app::App;

mod types;
mod pages;
mod components;
pub mod app;

pub static BASE_DIR: Lazy<PathBuf> = Lazy::new(|| {
    let dir = ProjectDirs::from("eu", "mq1", "CrabLauncher").unwrap().data_dir().to_path_buf();

    fs::create_dir_all(&dir).unwrap();

    dir
});

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "CrabLauncher",
        native_options,
        Box::new(|cc| Box::new(App::new(cc))),
    )
}
