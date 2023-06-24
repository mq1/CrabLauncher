// SPDX-FileCopyrightText: 2023 Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use iced::{
    widget::{column, text, vertical_space},
    Alignment, Element, Length,
};

use crate::Message;

pub fn view() -> Element<'static, Message> {
    column![
        vertical_space(Length::Fill),
        text("Todo").size(30),
        vertical_space(Length::Fill),
    ]
    .align_items(Alignment::Center)
    .into()
}
