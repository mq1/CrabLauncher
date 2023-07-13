// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuq01@pm.me>
// SPDX-License-Identifier: GPL-3.0-only

use iced::{
    widget::{column, text, vertical_space},
    Alignment, Command, Element, Length,
};

use crate::{pages::Page, Message};

pub struct ModrinthInstaller;

impl Page for ModrinthInstaller {
    type Message = Message;

    fn update(&mut self, _message: Message) -> Command<Message> {
        Command::none()
    }

    fn view(&self) -> Element<'static, Message> {
        column![
            vertical_space(Length::Fill),
            text("Todo").size(30),
            vertical_space(Length::Fill),
        ]
        .align_items(Alignment::Center)
        .into()
    }
}
