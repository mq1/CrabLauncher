// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuelquarneti@protonmail.com>
// SPDX-License-Identifier: GPL-3.0-only

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use iced::{Application, Settings};

use crate::launcher::Launcher;
use crate::message::Message;

mod icon;
mod info;
mod instances;
mod launcher;
mod message;
mod navbar;
mod pages;
mod style;
mod vanilla_installer;

#[tokio::main]
async fn main() -> iced::Result {
    Launcher::run(Settings::default())
}
