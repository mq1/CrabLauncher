// SPDX-FileCopyrightText: 2023 Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

mod instances;
mod settings;
mod style;

use std::{fs, path::PathBuf};

use anyhow::Result;
use directories::ProjectDirs;
use iced::{executor, Application, Command, Element, Settings as IcedSettings, Theme};
use instances::Instances;
use once_cell::sync::Lazy;
use settings::Settings;

pub static BASE_DIR: Lazy<PathBuf> = Lazy::new(|| {
    let proj_dirs = ProjectDirs::from("eu", "mq1", "icy-launcher").unwrap();
    proj_dirs.data_dir().to_path_buf()
});

pub fn main() -> Result<()> {
    if !BASE_DIR.exists() {
        fs::create_dir_all(BASE_DIR.as_path())?;
    }

    App::run(IcedSettings::default())?;
    Ok(())
}

#[derive(Debug, Clone)]
pub enum View {
    Instances,
    Settings,
}

struct App {
    view: View,
    instances: Instances,
    settings: Settings,
}

#[derive(Debug, Clone)]
pub enum Message {
    ChangeView(View),
    CheckForUpdates(bool),
}

impl Application for App {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        let instances = Instances::load();
        let settings = Settings::load().unwrap();

        (
            Self {
                view: View::Instances,
                instances,
                settings,
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
        }
    }

    fn view(&self) -> Element<Message> {
        match self.view {
            View::Instances => self.instances.view(),
            View::Settings => self.settings.view(),
        }
    }
}
