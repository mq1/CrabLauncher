// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use iced::{
    widget::{button, column, container, horizontal_space, pick_list, row, text, text_input},
    Element, Length,
};

use crate::{style, InstallerInfo, Message};

pub fn view(info: &InstallerInfo) -> Element<Message> {
    let heading = text("New instance").size(50);

    let instance_name = container(row![
        text("Instance name"),
        horizontal_space(Length::Fill),
        text_input("Instance name", &info.name, Message::NewInstanceNameChanged),
    ])
    .padding(10)
    .style(style::card());

    let version: Element<_> = match &info.vanilla_versions {
        Some(Ok(versions)) => pick_list(
            versions,
            info.selected_vanilla_version.clone(),
            Message::VanillaVersionSelected,
        )
        .into(),
        Some(Err(error)) => text(error).into(),
        None => text("Loading versions...").into(),
    };

    let version = container(row![
        text("Minecraft version"),
        horizontal_space(Length::Fill),
        version,
    ])
    .padding(10)
    .style(style::card());

    let create_button = button("Create instance").on_press(Message::CreateVanillaInstance);

    column![heading, instance_name, version, create_button]
        .padding(20)
        .spacing(20)
        .into()
}
