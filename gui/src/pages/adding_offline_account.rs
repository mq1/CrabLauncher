// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuelquarneti@protonmail.com>
// SPDX-License-Identifier: GPL-3.0-only

use iced::{Alignment, Element, Length, theme};
use iced::widget::{button, Column, container, text, text_input, vertical_space};
use crate::style;

use crate::types::messages::Message;

pub fn view(username: &str) -> Element<Message> {
    let title = text("Adding offline account").size(30);

    let username_input = text_input("Username", username)
        .width(200)
        .on_input(Message::OfflineAccountUsernameChanged);

    let add_button = button(container(text("Add")).padding(5))
        .on_press(Message::AddOfflineAccount)
        .style(style::circle_button(theme::Button::Primary));

    Column::new()
        .push(vertical_space(Length::Fill))
        .push(title)
        .push(username_input)
        .push(add_button)
        .push(vertical_space(Length::Fill))
        .width(Length::Fill)
        .spacing(20)
        .align_items(Alignment::Center)
        .into()
}
