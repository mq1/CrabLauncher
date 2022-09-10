// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

// On Windows platform, don't show a console when opening the app.
#![windows_subsystem = "windows"]

mod about;
mod accounts;
mod creating_instance;
mod install_runtime;
mod instance_name_selection;
mod instance_type_selection;
mod instance_version_selection;
mod instances;
mod lib;
mod news;
mod root;
mod runtimes;
mod settings;
mod loading_versions;

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
    InstanceTypeSelection,
    InstanceVersionSelection,
    LoadingVersions,
    InstanceNameSelection,
    CreatingInstance,
    Accounts,
    Runtimes,
    InstallRuntime,
    News,
    Settings,
    About,
}

#[derive(Data, Clone, Lens)]
pub struct NewInstanceState {
    available_minecraft_versions: Vector<lib::minecraft_version_manifest::Version>,
    shown_minecraft_versions: Vector<lib::minecraft_version_manifest::Version>,
    selected_version: Option<lib::minecraft_version_manifest::Version>,
    instance_type: lib::instances::InstanceType,
    instance_name: String,
    show_releases: bool,
    show_snapshots: bool,
    show_beta: bool,
    show_alpha: bool,
}

#[derive(Data, Clone, Lens)]
pub struct AppState {
    config: lib::launcher_config::LauncherConfig,
    current_view: View,
    instances: Vector<(String, lib::instances::InstanceInfo)>,
    new_instance_state: NewInstanceState,
    accounts: Vector<(lib::msa::AccountEntry, bool)>,
    active_account: Option<lib::msa::AccountEntry>,
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

    let new_instance_state = NewInstanceState {
        available_minecraft_versions: vector![],
        shown_minecraft_versions: vector![],
        selected_version: None,
        instance_type: lib::instances::InstanceType::Vanilla,
        instance_name: String::new(),
        show_releases: true,
        show_snapshots: false,
        show_beta: false,
        show_alpha: false,
    };

    let initial_state = AppState {
        config: lib::launcher_config::read()?,
        current_view: View::Instances,
        instances: lib::instances::list()?,
        new_instance_state,
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
