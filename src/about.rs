// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use iced::{
    widget::{button, column, horizontal_space, row, text, vertical_space},
    Element, Length,
};

use crate::Message;

const APP_NAME: &str = "Ice Launcher";
const APP_VERSION: &str = concat!("v", env!("CARGO_PKG_VERSION"));
const COPYRIGHT: &str = "Copyright © 2022 Manuel Quarneti";

pub struct AboutView;

impl AboutView {
    pub fn new() -> Self {
        Self
    }

    pub fn view(&self) -> Element<Message> {
        column![
            text(APP_NAME).size(50),
            vertical_space(Length::Units(20)),
            text(APP_VERSION),
            vertical_space(Length::Fill),
            row![
                button("Repository").on_press(Message::OpenRepository),
                horizontal_space(Length::Units(10)),
                button("License").on_press(Message::OpenLicense),
                horizontal_space(Length::Fill),
                text(COPYRIGHT),
            ],
        ]
        .padding(20)
        .into()
    }
}
