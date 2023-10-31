// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuelquarneti@protonmail.com>
// SPDX-License-Identifier: GPL-3.0-only

use crate::components::icon::Icon;
use iced::widget::{text, vertical_space, Column};
use iced::{Alignment, Element, Length};

use crate::types::messages::Message;

pub fn view(err: &str) -> Element<Message> {
    let error = text(err).size(30);

    Column::new()
        .push(vertical_space(Length::Fill))
        .push(Icon::AlertCircleOutline.view(64))
        .push(error)
        .push(vertical_space(Length::Fill))
        .width(Length::Fill)
        .align_items(Alignment::Center)
        .spacing(10)
        .into()
}
