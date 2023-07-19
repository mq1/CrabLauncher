// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuq01@pm.me>
// SPDX-License-Identifier: GPL-3.0-only

use iced::{
    Alignment,
    Element, Length, widget::{Column, text, vertical_space},
};

use crate::{Message, pages::Page};

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
        Column::new()
            .push(vertical_space(Length::Fill))
            .push(text(&self.text).size(30))
            .push(vertical_space(Length::Fill))
            .align_items(Alignment::Center)
            .width(Length::Fill)
            .into()
    }
}
