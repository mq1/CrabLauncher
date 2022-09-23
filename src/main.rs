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
mod loading_versions;
mod news;
mod root;
mod runtimes;
mod settings;

use std::fs;

use color_eyre::eyre::Result;
use druid::{im::Vector, AppLauncher, Data, Lens, WindowDesc};
use lib::BASE_DIR;
use strum_macros::Display;

#[derive(PartialEq, Eq, Data, Clone, Copy, Display, Default)]
enum View {
    #[default]
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

#[derive(Data, Clone, Lens, Default)]
pub struct NewInstanceState {
    available_minecraft_versions: Vector<lib::minecraft_version_manifest::Version>,
    selected_version: Option<lib::minecraft_version_manifest::Version>,
    instance_type: lib::instances::InstanceType,
    instance_name: String,
}

#[derive(Data, Clone, Lens, Default)]
pub struct AppState {
    is_update_available: bool,
    config: lib::launcher_config::LauncherConfig,
    current_view: View,
    instances: Vector<(String, lib::instances::InstanceInfo)>,
    new_instance_state: NewInstanceState,
    accounts: Vector<lib::msa::Account>,
    active_account: Option<lib::msa::Account>,
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

    let initial_state = smol::block_on(async move {
        let config = lib::launcher_config::read();
        let instances = lib::instances::list();
        let accounts = lib::accounts::read();
        let active_account = lib::accounts::get_active();
        let installed_runtimes = lib::runtime_manager::list();

        AppState {
            config: config.await.unwrap(),
            instances: instances.await.unwrap(),
            accounts: Vector::from(accounts.await.unwrap().accounts),
            active_account: active_account.await.unwrap(),
            installed_runtimes: installed_runtimes.await.unwrap(),
            ..Default::default()
        }
    });

    let launcher = AppLauncher::with_window(window);

    // Spawn a task to check for updates.
    if initial_state.config.automatically_check_for_updates {
        let event_sink = launcher.get_external_handle();
        smol::spawn(check_for_updates(event_sink)).detach();
    }

    launcher.log_to_console().launch(initial_state)?;

    Ok(())
}

async fn check_for_updates(event_sink: druid::ExtEventSink) {
    match lib::launcher_updater::check_for_updates().await {
        Ok(update) => {
            if update.is_some() {
                event_sink.add_idle_callback(|data: &mut AppState| {
                    data.is_update_available = true;
                })
            }
        }
        Err(_) => {}
    }
}
