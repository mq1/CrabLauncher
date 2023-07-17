// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuq01@pm.me>
// SPDX-License-Identifier: GPL-3.0-only

use iced::{
    theme,
    widget::{
        button, column, container, horizontal_space, row, text, toggler, vertical_space, Column,
    },
    Alignment, Command, Element, Length,
};

use crate::{components::icons, pages::Page, style, util::settings::Settings};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Message {
    SetCheckForUpdates(bool),
    SaveSettings,
}

impl Page for Settings {
    type Message = Message;

    fn update(&mut self, message: Message) -> Command<Message> {
        let ret = Command::none();

        match message {
            Message::SetCheckForUpdates(value) => {
                self.check_for_updates = value;
            }
            Message::SaveSettings => {
                self.save().unwrap();
            }
        }

        ret
    }

    fn view(&self) -> Element<Message> {
        let mut settings = Column::new().padding(10);

        #[cfg(feature = "updater")]
        {
            let check_for_updates = toggler(
                "Automatically check for updates".to_owned(),
                self.check_for_updates,
                Message::SetCheckForUpdates,
            );

            settings = settings.push(check_for_updates);
        }

        let save_button = button(
            row![text(" Save "), icons::view(icons::CONTENT_SAVE_OUTLINE)]
                .padding(5)
                .align_items(Alignment::Center),
        )
        .style(style::circle_button(theme::Button::Positive))
        .on_press(Message::SaveSettings);

        column![
            text("Settings").size(30),
            container(settings).style(style::card()),
            vertical_space(Length::Fill),
            row![horizontal_space(Length::Fill), save_button]
        ]
        .spacing(10)
        .padding(10)
        .into()
    }
}
