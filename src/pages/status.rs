// SPDX-FileCopyrightText: 2023 Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use iced::{
    widget::{column, text, vertical_space},
    Alignment, Element, Length,
};

use crate::{pages::Page, Message};

pub struct Status {
    pub text: String,
}

impl Status {
    pub fn new() -> Self {
        Self {
            text: String::new(),
        }
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
            vertical_space(Length::Fill),
        ]
        .align_items(Alignment::Center)
        .width(Length::Fill)
        .into()
    }
}
