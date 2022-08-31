// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

// On Windows platform, don't show a console when opening the app.
#![windows_subsystem = "windows"]

mod about;
mod accounts;
mod create_instance;
mod install_runtime;
mod instances;
mod lib;
mod news;
mod root;
mod runtimes;
mod settings;

use std::fs;

use color_eyre::eyre::Result;
use druid::{
    im::{vector, Vector},
    AppLauncher, Data, Lens, WindowDesc,
};
use lib::BASE_DIR;
use strum_macros::Display;

#[derive(PartialEq, Eq, Data, Clone, Copy, Display)]
enum View {
    Instances,
    CreateInstance,
    Accounts,
    Runtimes,
    InstallRuntime,
    News,
    Settings,
    About,
}

#[derive(Data, Clone, Lens)]
pub struct AppState {
    config: lib::launcher_config::LauncherConfig,
    current_view: View,
    instances: Vector<(String, lib::instances::InstanceInfo)>,
    accounts: Vector<(String, lib::accounts::Account, bool)>,
    active_account: Option<(String, lib::accounts::Account)>,
    news: Vector<(String, String)>,
    installed_runtimes: Vector<String>,
    available_runtimes: Vector<i32>,
    installing_runtime: bool,
}

fn main() -> Result<()> {
    color_eyre::install()?;

    fs::create_dir_all(BASE_DIR.as_path()).expect("Could not create base directory");

    let window = WindowDesc::new(root::build_widget())
        .title("Ice Launcher")
        .window_size((800.0, 600.0));

    let initial_state = AppState {
        config: lib::launcher_config::read()?,
        current_view: View::Instances,
        instances: lib::instances::list()?,
        accounts: lib::accounts::list()?,
        active_account: lib::accounts::get_active()?,
        news: vector![],
        installed_runtimes: lib::runtime_manager::list()?,
        available_runtimes: vector![],
        installing_runtime: false,
    };

    AppLauncher::with_window(window)
        .log_to_console()
        .launch(initial_state)
        .expect("Launch failed");

    Ok(())
}
