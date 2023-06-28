// SPDX-FileCopyrightText: 2023 Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use iced::{
    widget::{button, column, container, horizontal_space, row, text, toggler, vertical_space},
    Command, Element, Length,
};

use crate::{components::icons, style, util};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Message {
    SetCheckForUpdates(bool),
    SaveSettings,
}

pub struct SettingsPage {
    pub settings: util::settings::Settings,
}

impl SettingsPage {
    pub fn new() -> Self {
        Self {
            settings: util::settings::Settings::load().unwrap(),
        }
    }

    pub fn update(&mut self, message: Message) -> Command<Message> {
        let ret = Command::none();

        match message {
            Message::SetCheckForUpdates(value) => {
                self.settings.check_for_updates = value;
            }
            Message::SaveSettings => {
                self.settings.save().unwrap();
            }
        }

        ret
    }

    pub fn view(&self) -> Element<Message> {
        let check_for_updates = toggler(
            "Check for updates".to_owned(),
            self.settings.check_for_updates,
            Message::SetCheckForUpdates,
        );

        let save_button = button(icons::content_save())
            .style(style::circle_button())
            .on_press(Message::SaveSettings);

        column![
            text("Settings").size(30),
            container(column![check_for_updates].padding(10)).style(style::card()),
            vertical_space(Length::Fill),
            row![horizontal_space(Length::Fill), save_button]
        ]
        .spacing(10)
        .padding(10)
        .into()
    }
}
