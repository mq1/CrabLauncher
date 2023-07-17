// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuq01@pm.me>
// SPDX-License-Identifier: GPL-3.0-only

use iced::{
    theme,
    widget::{button, column, horizontal_space, row, text, vertical_space},
    Alignment, Command, Element, Length,
};

use crate::{components::icons, pages::Page, style, util::instances::Instance, Message};

impl Page for Instance {
    type Message = Message;

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        let name = text(self.name.clone()).size(50);
        let play_button = button(
            row![text("Launch").size(30), icons::view(icons::ROCKET_LINE)]
                .padding(20)
                .spacing(10)
                .align_items(Alignment::Center),
        )
        .on_press(Message::LaunchInstance(self.to_owned()))
        .style(style::circle_button(theme::Button::Primary));

        let delete_button = button(
            row![
                text(" Delete instance "),
                icons::view(icons::DELETE_BIN_LINE)
            ]
            .padding(5)
            .align_items(Alignment::Center),
        )
        .style(style::circle_button(theme::Button::Destructive))
        .on_press(Message::DeleteInstance(self.name.clone()));

        let edit_button = button(
            row![text(" Edit instance "), icons::view(icons::SETTINGS_LINE)]
                .padding(5)
                .align_items(Alignment::Center),
        )
        .style(style::circle_button(theme::Button::Primary));

        column![
            vertical_space(Length::Fill),
            name,
            play_button,
            vertical_space(Length::Fill),
            row![horizontal_space(Length::Fill), delete_button, edit_button]
                .spacing(10)
                .padding(10),
        ]
        .align_items(Alignment::Center)
        .width(Length::Fill)
        .spacing(20)
        .into()
    }
}
