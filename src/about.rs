// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use iced::{
    widget::{button, column, horizontal_space, image, row, text, vertical_space},
    Alignment, Element, Length,
};

use crate::Message;

const APP_NAME: &str = "Ice Launcher";
const APP_VERSION: &str = concat!("v", env!("CARGO_PKG_VERSION"));
const COPYRIGHT: &str = "Copyright © 2022 Manuel Quarneti";
const LOGO_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/ice-launcher.png");
const REPOSITORY: &str = "https://github.com/mq1/ice-launcher";
const LICENSE: &str = "https://github.com/mq1/ice-launcher/blob/main/COPYING";

pub struct AboutView;

impl AboutView {
    pub fn new() -> Self {
        Self
    }

    pub fn view(&self) -> Element<Message> {
        column![
            vertical_space(Length::Fill),
            image(LOGO_PATH).height(Length::Units(200)),
            text(APP_NAME).size(50),
            text(APP_VERSION),
            text("Made with <3 in Rust by Manuel Quarneti"),
            vertical_space(Length::Fill),
            row![
                button("Repository").on_press(Message::OpenURL(REPOSITORY.to_string())),
                button("License").on_press(Message::OpenURL(LICENSE.to_string())),
                horizontal_space(Length::Fill),
                text(COPYRIGHT),
            ]
            .spacing(10),
        ]
        .padding(20)
        .spacing(10)
        .align_items(Alignment::Center)
        .into()
    }
}
