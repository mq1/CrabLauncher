// SPDX-FileCopyrightText: 2023 Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use iced::{
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
            row![text("Launch").size(30), icons::rocket()]
                .padding(20)
                .spacing(10)
                .align_items(Alignment::Center),
        )
        .style(style::circle_button());

        let edit_button = button(
            row![text(" Edit instance"), icons::cog(20.)]
                .padding(5)
                .spacing(5)
                .align_items(Alignment::Center),
        )
        .style(style::circle_button());

        column![
            vertical_space(Length::Fill),
            name,
            play_button,
            vertical_space(Length::Fill),
            row![horizontal_space(Length::Fill), edit_button].padding(10),
        ]
        .align_items(Alignment::Center)
        .width(Length::Fill)
        .spacing(20)
        .into()
    }
}
