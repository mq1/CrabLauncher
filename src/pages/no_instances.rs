// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuq01@pm.me>
// SPDX-License-Identifier: GPL-3.0-only

use iced::{
    widget::{column, row, text, vertical_space},
    Alignment, Command, Element,
};

use crate::{components::icons, pages::Page, Message};

pub struct NoInstances;

impl Page for NoInstances {
    type Message = Message;

    fn update(&mut self, _message: Message) -> Command<Message> {
        Command::none()
    }

    fn view(&self) -> Element<'static, Message> {
        column![
            vertical_space(55),
            row![
                icons::arrow_left(),
                text("You don't have any instances yet. Create one!").size(25)
            ]
            .align_items(Alignment::Center)
            .spacing(10)
        ]
        .padding(10)
        .into()
    }
}
