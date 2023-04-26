// SPDX-FileCopyrightText: 2023 Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use iced::{
    widget::{button, column, horizontal_space, image, row, text, vertical_space},
    Alignment, Element, Length,
};

use crate::{assets, Message, View};

const APP_NAME: &str = env!("CARGO_PKG_NAME");
const APP_VERSION: &str = concat!("v", env!("CARGO_PKG_VERSION"));
const LICENSE: &str = concat!(env!("CARGO_PKG_LICENSE"), " Licensed");
const COPYRIGHT: &str = concat!("Copyright © 2023 ", env!("CARGO_PKG_AUTHORS"));
const REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");

pub fn view() -> Element<'static, Message> {
    let header = row![
        horizontal_space(Length::Fill),
        button("Back").on_press(Message::ChangeView(View::Instances))
    ];

    let logo_handle = image::Handle::from_memory(assets::LOGO_PNG);
    let logo = image::viewer(logo_handle).height(200);

    column![
        header,
        vertical_space(Length::Fill),
        logo,
        text(APP_NAME).size(50),
        text(APP_VERSION),
        vertical_space(Length::Fill),
        row![
            button("Repository").on_press(Message::OpenURL(REPOSITORY.to_string())),
            horizontal_space(Length::Fill),
            text(LICENSE.to_owned() + " · " + COPYRIGHT),
        ]
        .spacing(10),
    ]
    .spacing(10)
    .padding(10)
    .align_items(Alignment::Center)
    .into()
}
