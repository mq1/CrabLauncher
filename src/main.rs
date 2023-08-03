// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuq01@pm.me>
// SPDX-License-Identifier: GPL-3.0-only

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use iced::{Application, Color, Command, Element, executor, Settings, Subscription, Theme, theme};

use crate::types::launcher::Launcher;
use crate::types::messages::Message;

mod components;
mod pages;
mod style;
mod subscriptions;
mod types;
mod util;
pub mod assets;

pub fn main() -> iced::Result {
    let mut settings = Settings::default();
    let icon = iced::window::icon::from_file_data(assets::LOGO_PNG, None).unwrap();
    settings.window.icon = Some(icon);

    Launcher::run(settings)
}

impl Application for Launcher {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        Launcher::new()
    }

    fn title(&self) -> String {
        self.name.to_owned()
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        self.update(message)
    }

    fn view(&self) -> Element<Message> {
        pages::root::view(self)
    }

    fn theme(&self) -> Self::Theme {
        Theme::custom(theme::Palette {
            primary: Color::from_rgb8(192, 101, 33),
            ..Theme::Dark.palette()
        })
    }

    fn subscription(&self) -> Subscription<Message> {
        self.subscription()
    }
}
