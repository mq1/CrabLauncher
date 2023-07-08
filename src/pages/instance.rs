// SPDX-FileCopyrightText: 2023 Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use iced::{
    widget::{button, column, container, row, text, vertical_space},
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

        let container = container(
            column![name, play_button,]
                .spacing(50)
                .padding(50)
                .align_items(Alignment::Center)
                .width(Length::Fill),
        )
        .style(style::card())
        .width(Length::Fill);

        column![
            vertical_space(Length::Fill),
            container,
            vertical_space(Length::Fill)
        ]
        .align_items(Alignment::Center)
        .width(Length::Fill)
        .padding(200)
        .into()
    }
}
