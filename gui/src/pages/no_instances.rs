// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuelquarneti@protonmail.com>
// SPDX-License-Identifier: GPL-3.0-only

use iced::{
    widget::{text, vertical_space, Column, Row},
    Alignment, Element,
};

use crate::{components::icon::Icon, Message};

pub fn view() -> Element<'static, Message> {
    Column::new()
        .push(vertical_space(55))
        .push(
            Row::new()
                .push(Icon::ArrowLeft.view(24))
                .push(text("You don't have any instances yet. Create one!").size(25))
                .align_items(Alignment::Center)
                .spacing(10),
        )
        .padding(10)
        .into()
}
