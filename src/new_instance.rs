// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use iced::{
    widget::{column, container, horizontal_space, pick_list, row, text, text_input},
    Element, Length,
};

use crate::{lib, style, Message};

pub fn view<'a>(
    name: &'a str,
    available_versions: &'a Option<Result<Vec<lib::minecraft_version_manifest::Version>, String>>,
    selected_version: &'a Option<lib::minecraft_version_manifest::Version>,
) -> Element<'a, Message> {
    let heading = text("New instance").size(50);

    let instance_name = container(row![
        text("Instance name"),
        horizontal_space(Length::Fill),
        text_input("Instance name", name, Message::NewInstanceNameChanged),
    ])
    .padding(10)
    .style(style::card());

    let version: Element<_> = match available_versions {
        Some(Ok(versions)) => pick_list(
            versions,
            selected_version.to_owned(),
            Message::VersionSelected,
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

    column![heading, instance_name, version]
        .padding(20)
        .spacing(20)
        .into()
}
