// SPDX-FileCopyrightText: 2023 Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use iced::{
    widget::{
        button, column, container, horizontal_space, pick_list, row, text, text_input,
        vertical_space,
    },
    Element, Length,
};

use crate::{style, Message};

pub fn view<'a>(
    installer_name: &'a str,
    versions: &'a Vec<String>,
    selected_version: Option<String>,
    name: &'a str,
) -> Element<'a, Message> {
    let title = text(installer_name).size(30);

    let name_text = text("Instance name");
    let name = text_input("", name).on_input(Message::ChangeInstanceName);
    let choose_name = column![name_text, name].spacing(10).padding(10);
    let choose_name = container(choose_name)
        .width(Length::Fill)
        .style(style::card());

    let version_text = text("Select version");
    let version_picker = pick_list(versions, selected_version, Message::SelectVersion);
    let select_version = column![version_text, version_picker]
        .spacing(10)
        .padding(10);
    let select_version = container(select_version)
        .width(Length::Fill)
        .style(style::card());

    let create_button = button("Create").style(style::circle_button());
    let footer = row![horizontal_space(Length::Fill), create_button];

    column![
        title,
        choose_name,
        select_version,
        vertical_space(Length::Fill),
        footer,
    ]
    .spacing(10)
    .padding(10)
    .into()
}
