// SPDX-FileCopyrightText: 2023 Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use iced::{
    widget::{button, column, horizontal_space, image, row, text, vertical_space, Image},
    Alignment, Element, Length,
};

use crate::{
    components::{assets, icons},
    style, Message,
};

const APP_NAME: &str = env!("CARGO_PKG_NAME");
const APP_VERSION: &str = concat!("v", env!("CARGO_PKG_VERSION"));
const LICENSE: &str = concat!(env!("CARGO_PKG_LICENSE"), " Licensed");
const COPYRIGHT: &str = concat!("Copyright © 2023 ", env!("CARGO_PKG_AUTHORS"));
const REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");

pub fn view() -> Element<'static, Message> {
    let logo_handle = image::Handle::from_memory(assets::LOGO_PNG);
    let logo = Image::new(logo_handle).height(200);

    let repo_button = button(
        row!["Repository ", icons::github()]
            .align_items(Alignment::Center)
            .padding([0, 0, 0, 5]),
    )
    .style(style::circle_button())
    .on_press(Message::OpenURL(REPOSITORY.to_string()));

    column![
        vertical_space(Length::Fill),
        logo,
        text(APP_NAME).size(50),
        text(APP_VERSION),
        text(LICENSE),
        text(COPYRIGHT),
        vertical_space(Length::Fill),
        row![horizontal_space(Length::Fill), repo_button],
    ]
    .spacing(10)
    .padding(10)
    .align_items(Alignment::Center)
    .into()
}
