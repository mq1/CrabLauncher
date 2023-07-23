// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuq01@pm.me>
// SPDX-License-Identifier: GPL-3.0-only

use iced::{Alignment, Element, Length, theme};
use iced::widget::{button, Column, container, text, vertical_space};

use crate::style;
use crate::types::login::Login;
use crate::types::messages::Message;

pub fn view(login: &Login) -> Element<Message> {
    let message = text(format!(
        "Please open up {} in a browser and put in the code {} to proceed with login",
        login.url, login.code
    ))
        .size(20);

    let open_button = button(container(text("Open page and copy code")).padding(5))
        .style(style::circle_button(theme::Button::Primary))
        .on_press(Message::OpenLoginUrl);

    return Column::new()
        .push(vertical_space(Length::Fill))
        .push(message)
        .push(open_button)
        .push(vertical_space(Length::Fill))
        .width(Length::Fill)
        .spacing(20)
        .align_items(Alignment::Center)
        .into();
}
