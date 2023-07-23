// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuq01@pm.me>
// SPDX-License-Identifier: GPL-3.0-only

use iced::{
    Alignment,
    Element, widget::{Column, Row, text, vertical_space},
};

use crate::{components::icons, Message};

pub fn view() -> Element<'static, Message> {
    Column::new()
        .push(vertical_space(55))
        .push(
            Row::new()
                .push(icons::view(icons::ARROW_LEFT))
                .push(text("You don't have any instances yet. Create one!").size(25))
                .align_items(Alignment::Center)
                .spacing(10)
        )
        .padding(10)
        .into()
}
