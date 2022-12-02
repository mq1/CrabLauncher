// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use iced::{
    widget::{button, column, container, horizontal_space, pick_list, row, text, text_input},
    Command, Element, Length,
};
use mclib::minecraft_version_manifest::Version;

use crate::style;

#[derive(Debug, Clone)]
pub enum Message {
    FetchVersions,
    FetchedVersions(Result<Vec<Version>, String>),
    NewInstanceNameChanged(String),
    VersionSelected(Version),
    CreateInstance,
}

pub struct VanillaInstaller {
    pub name: String,
    pub available_versions: Option<Result<Vec<mclib::minecraft_version_manifest::Version>, String>>,
    pub selected_version: Option<mclib::minecraft_version_manifest::Version>,
}

impl VanillaInstaller {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            available_versions: None,
            selected_version: None,
        }
    }

    pub fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::FetchVersions => {
                return Command::perform(
                    async {
                        mclib::minecraft_version_manifest::fetch_versions()
                            .map_err(|e| e.to_string())
                    },
                    Message::FetchedVersions,
                );
            }
            Message::FetchedVersions(versions) => {
                self.available_versions = Some(versions);
            }
            Message::NewInstanceNameChanged(name) => {
                self.name = name;
            }
            Message::VersionSelected(version) => {
                self.selected_version = Some(version);
            }
            Message::CreateInstance => {
                
            }
        }

        Command::none()
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
