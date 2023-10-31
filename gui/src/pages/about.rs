// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuelquarneti@protonmail.com>
// SPDX-License-Identifier: GPL-3.0-only

use iced::widget::image;
use iced::{
    theme,
    widget::{button, horizontal_space, text, vertical_space, Column, Row},
    Alignment, Element, Length,
};

use crate::components::icon::Icon;
use crate::{style, Message, LOGO_PNG};

const APP_VERSION: &str = concat!("v", env!("CARGO_PKG_VERSION"));
const LICENSE: &str = concat!(env!("CARGO_PKG_LICENSE"), " Licensed");
const COPYRIGHT: &str = concat!("Copyright © 2023 ", env!("CARGO_PKG_AUTHORS"));
const REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");

pub fn view(launcher_name: &'static str) -> Element<'static, Message> {
    let logo = image::Handle::from_memory(LOGO_PNG);
    let logo = image(logo).width(100).height(100);

    let repo_button = button(
        Row::new()
            .push(text(" Repository "))
            .push(Icon::Github.view(24))
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
