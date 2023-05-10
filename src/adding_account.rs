// SPDX-FileCopyrightText: 2023 Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use iced::{
    widget::{button, column, text, vertical_space},
    Alignment, Element, Length,
};

use crate::{style, Message};

pub fn view<'a>(url: &'a str, code: &'a str) -> Element<'a, Message> {
    let message = text(format!(
        "Please open up {url} in a browser and put in the code {code} to proceed with login"
    ))
    .size(20);

    let open_button = button("Open page and copy code")
        .style(style::circle_button())
        .on_press(Message::Login(url.to_owned(), code.to_owned()));

    column![
        vertical_space(Length::Fill),
        message,
        open_button,
        vertical_space(Length::Fill),
    ]
    .width(Length::Fill)
    .spacing(10)
    .align_items(Alignment::Center)
    .into()
}
