// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuq01@pm.me>
// SPDX-License-Identifier: GPL-3.0-only

use iced::{
    Alignment,
    Element, Length, widget::{Column, text, vertical_space},
};

use crate::Message;

pub fn view(status: &str) -> Element<Message> {
    Column::new()
        .push(vertical_space(Length::Fill))
        .push(text(status).size(30))
        .push(vertical_space(Length::Fill))
        .align_items(Alignment::Center)
        .width(Length::Fill)
        .into()
}
