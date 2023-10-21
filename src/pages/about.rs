// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuelquarneti@protonmail.com>
// SPDX-License-Identifier: GPL-3.0-only

use iced::{
    Alignment,
    Element,
    Length, theme, widget::{button, Column, horizontal_space, row, Row, text, vertical_space},
};

use crate::{assets, components::icons, Message, style};

const APP_VERSION: &str = concat!("v", env!("CARGO_PKG_VERSION"));
const LICENSE: &str = concat!(env!("CARGO_PKG_LICENSE"), " Licensed");
const COPYRIGHT: &str = concat!("Copyright © 2023 ", env!("CARGO_PKG_AUTHORS"));
const REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");

pub fn view(launcher_name: &'static str) -> Element<'static, Message> {
    let logo = icons::view_png(assets::LOGO_PNG, 128);

    let repo_button = button(
        row![" Repository ", icons::view(icons::GITHUB)]
            .align_items(Alignment::Center)
            .padding(5),
    )
        .style(style::circle_button(theme::Button::Primary))
        .on_press(Message::OpenURL(REPOSITORY.to_string()));

    let footer = Row::new()
        .push(horizontal_space(Length::Fill))
        .push(repo_button);

    Column::new()
        .push(vertical_space(Length::Fill))
        .push(logo)
        .push(text(launcher_name).size(50))
        .push(text(APP_VERSION))
        .push(text(LICENSE))
        .push(text(COPYRIGHT))
        .push(vertical_space(Length::Fill))
        .push(footer)
        .spacing(10)
        .padding(10)
        .align_items(Alignment::Center)
        .into()
}
