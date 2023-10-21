// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuelquarneti@protonmail.com>
// SPDX-License-Identifier: GPL-3.0-only

use iced::{Alignment, Element, Length};
use iced::widget::{Column, text, vertical_space};

use crate::components::icons;
use crate::types::messages::Message;

pub fn view(err: &str) -> Element<Message> {
    let error = text(err).size(30);
    let error_icon = icons::view_custom(icons::ALERT_CIRCLE_OUTLINE, 32);

    Column::new()
        .push(vertical_space(Length::Fill))
        .push(error_icon)
        .push(error)
        .push(vertical_space(Length::Fill))
        .width(Length::Fill)
        .align_items(Alignment::Center)
        .spacing(10)
        .into()
}
