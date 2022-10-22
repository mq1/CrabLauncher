// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

// On Windows platform, don't show a console when opening the app.
#![windows_subsystem = "windows"]

mod about;
mod accounts;
mod instance_name_selection;
mod instance_type_selection;
mod instance_version_selection;
mod instances;
mod lib;
mod loading;
mod news;
mod progress;
mod view;
mod settings;
mod navbar;

use std::{fs, process::exit};

use color_eyre::eyre::Result;
use druid::{im::Vector, AppDelegate, AppLauncher, Data, Lens, WindowDesc};
use lib::BASE_DIR;

#[derive(PartialEq, Eq, Data, Clone, Copy, Default)]
enum View {
    #[default]
    Instances,
    Loading,
    Progress,
    InstanceTypeSelection,
    InstanceVersionSelection,
    InstanceNameSelection,
    Accounts,
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
    loading_message: String,
    current_progress: f64,
    config: lib::launcher_config::LauncherConfig,
    current_view: View,
    instances: Vector<lib::instances::Instance>,
    new_instance_state: NewInstanceState,
    accounts: Vector<lib::msa::Account>,
    active_account: Option<lib::msa::Account>,
    news: lib::minecraft_news::News,
}

struct Delegate;

impl AppDelegate<AppState> for Delegate {
    fn command(
        &mut self,
        _ctx: &mut druid::DelegateCtx,
        _target: druid::Target,
        cmd: &druid::Command,
        _data: &mut AppState,
        _env: &druid::Env,
    ) -> druid::Handled {
        if let Some(_) = cmd.get(druid::commands::CLOSE_WINDOW) {
            exit(0);
        }

        druid::Handled::No
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    fs::create_dir_all(BASE_DIR.as_path()).expect("Could not create base directory");

    let window = WindowDesc::new(view::build_widget())
        .title("Ice Launcher")
        .window_size((800.0, 600.0));

    let initial_state = {
        let config = lib::launcher_config::read();
        let instances = lib::instances::list();
        let accounts = lib::accounts::read();
        let active_account = lib::accounts::get_active();

        AppState {
            config: config.await?,
            instances: instances.await?,
            accounts: accounts.await?.accounts,
            active_account: active_account.await?,
            ..Default::default()
        }
    };

    let launcher = AppLauncher::with_window(window);

    // Spawn a task to check for updates.
    if initial_state.config.automatically_check_for_updates {
        let event_sink = launcher.get_external_handle();
        tokio::spawn(lib::launcher_updater::check_for_updates(event_sink));
    }

    launcher
        .delegate(Delegate {})
        .log_to_console()
        .launch(initial_state)?;

    Ok(())
}
