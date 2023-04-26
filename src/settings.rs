// SPDX-FileCopyrightText: 2023 Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use iced::{
    widget::{button, column, horizontal_space, row, text, toggler},
    Element, Length,
};

use crate::{Message, View};

pub struct Settings {
    pub check_for_updates: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            check_for_updates: true,
        }
    }
}

impl Settings {
    pub fn load() -> Self {
        Self::default()
    }

    pub fn view(&self) -> Element<Message> {
        let check_for_updates = toggler(
            "Check for updates".to_owned(),
            self.check_for_updates,
            Message::CheckForUpdates,
        );

        let header = row![
            text("Settings").size(30),
            horizontal_space(Length::Fill),
            button("Back").on_press(Message::ChangeView(View::Instances))
        ];

        column![header, check_for_updates]
            .spacing(10)
            .padding(10)
            .into()
    }
}
