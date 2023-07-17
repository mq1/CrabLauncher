// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuq01@pm.me>
// SPDX-License-Identifier: GPL-3.0-only

use iced::{
    widget::{column, text, vertical_space},
    Alignment, Element, Length,
};

use crate::{pages::Page, Message};

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Status {
    pub text: String,
}

impl Status {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Page for Status {
    type Message = Message;

    fn update(&mut self, _: Message) -> iced::Command<Message> {
        iced::Command::none()
    }

    fn view(&self) -> Element<Message> {
        column![
            vertical_space(Length::Fill),
            text(&self.text).size(30),
            vertical_space(Length::Fill)
        ]
        .align_items(Alignment::Center)
        .width(Length::Fill)
        .into()
    }
}
