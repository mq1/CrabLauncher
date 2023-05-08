// SPDX-FileCopyrightText: 2023 Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

mod about;
mod accounts;
mod assets;
mod icons;
mod instances;
mod settings;
mod style;
mod util;

use std::{fs, path::PathBuf};

use anyhow::Result;
use directories::ProjectDirs;
use iced::{executor, Application, Command, Element, Settings, Theme};
use once_cell::sync::Lazy;

pub static BASE_DIR: Lazy<PathBuf> = Lazy::new(|| {
    ProjectDirs::from("eu", "mq1", "icy-launcher")
        .unwrap()
        .data_dir()
        .to_path_buf()
});

pub fn main() -> Result<()> {
    if !BASE_DIR.exists() {
        fs::create_dir_all(BASE_DIR.as_path())?;
    }

    App::run(Settings::default())?;
    Ok(())
}

#[derive(Debug, Clone)]
pub enum View {
    Instances,
    Settings,
    About,
    Accounts,
}

struct App {
    view: View,
    instances: util::instances::Instances,
    settings: util::settings::Settings,
    accounts: util::accounts::Accounts,
}

#[derive(Debug, Clone)]
pub enum Message {
    ChangeView(View),
    CheckForUpdates(bool),
    SaveSettings,
    OpenURL(String),
    SelectAccount(util::accounts::Account),
}

impl Application for App {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let instances = <util::instances::Instances as util::instances::InstancesExt>::load();
        let settings = util::settings::Settings::load().unwrap();

        (
            Self {
                view: View::Instances,
                instances,
                settings,
                accounts: util::accounts::Accounts::load().unwrap(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Icy Launcher")
    }

    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::ChangeView(view) => {
                self.view = view;
                Command::none()
            }
            Message::CheckForUpdates(value) => {
                self.settings.check_for_updates = value;
                Command::none()
            }
            Message::SaveSettings => {
                self.settings.save().unwrap();
                Command::none()
            }
            Message::OpenURL(url) => {
                open::that(url).unwrap();
                Command::none()
            }
            Message::SelectAccount(account) => {
                self.accounts.set_active_account(account).unwrap();
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        match self.view {
            View::Instances => instances::view(&self.instances, &self.accounts.active),
            View::Settings => settings::view(&self.settings),
            View::About => about::view(),
            View::Accounts => accounts::view(&self.accounts),
        }
    }
}
