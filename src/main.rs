// SPDX-FileCopyrightText: 2023 Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

mod instances;
mod settings;
mod style;

use iced::{executor, Application, Command, Element, Settings as IcedSettings, Theme};
use instances::Instances;
use settings::Settings;

pub fn main() -> iced::Result {
    App::run(IcedSettings::default())
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
        let settings = Settings::load();

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
