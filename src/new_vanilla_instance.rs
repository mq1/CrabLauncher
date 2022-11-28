// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use iced::{
    widget::{button, column, container, horizontal_space, pick_list, row, text, text_input},
    Element, Length,
};

use crate::{util, style, Message};

pub struct NewVanillaInstance {
    pub name: String,
    pub available_versions: Option<Result<Vec<util::minecraft_version_manifest::Version>, String>>,
    pub selected_version: Option<util::minecraft_version_manifest::Version>,
}

impl NewVanillaInstance {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            available_versions: None,
            selected_version: None,
        }
    }

    pub async fn fetch_versions() -> Result<Vec<util::minecraft_version_manifest::Version>, String> {
        util::minecraft_version_manifest::fetch_versions().map_err(|e| e.to_string())
    }

    pub fn view(&self) -> Element<Message> {
        let heading = text("New instance").size(50);

        let instance_name = container(row![
            text("Instance name"),
            horizontal_space(Length::Fill),
            text_input("Instance name", &self.name, Message::NewInstanceNameChanged),
        ])
        .padding(10)
        .style(style::card());

        let version: Element<_> = match &self.available_versions {
            Some(Ok(versions)) => pick_list(
                versions,
                self.selected_version.to_owned(),
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

        let create_button = button("Create instance").on_press(Message::CreateInstance);

        column![heading, instance_name, version, create_button]
            .padding(20)
            .spacing(20)
            .into()
    }
}
