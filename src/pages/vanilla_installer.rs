// SPDX-FileCopyrightText: 2023 Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use iced::{
    futures::TryFutureExt,
    widget::{
        button, column, container, horizontal_space, pick_list, row, text, text_input,
        vertical_space,
    },
    Command, Element, Length,
};

use crate::{
    pages::Page,
    style,
    util::{self, instances::Instances},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Message {
    GetVersions,
    GotVersions(Result<Vec<util::vanilla_installer::Version>, String>),
    ChangeName(String),
    SelectVersion(util::vanilla_installer::Version),
    Create(Option<Instances>),
    CreatedInstance(Result<Instances, String>),
}

pub struct VanillaInstaller {
    pub versions: Vec<util::vanilla_installer::Version>,
    pub selected_version: Option<util::vanilla_installer::Version>,
    pub name: String,
}

impl VanillaInstaller {
    pub fn new() -> Self {
        Self {
            versions: Vec::new(),
            selected_version: None,
            name: String::new(),
        }
    }
}

impl Page for VanillaInstaller {
    type Message = Message;

    fn update(&mut self, message: Message) -> Command<Message> {
        let mut ret = Command::none();

        match message {
            Message::GetVersions => {
                ret = Command::perform(
                    util::vanilla_installer::get_versions().map_err(|e| e.to_string()),
                    Message::GotVersions,
                );
            }
            Message::GotVersions(Ok(versions)) => {
                self.versions = versions;
            }
            Message::GotVersions(Err(err)) => {
                eprintln!("Error: {}", err);
            }
            Message::ChangeName(name) => {
                self.name = name;
            }
            Message::SelectVersion(version) => {
                self.selected_version = Some(version);
            }
            Message::Create(instances) => {
                let name = self.name.clone();
                let version = self.selected_version.clone().unwrap();
                let instances = instances.unwrap();

                ret = Command::perform(
                    async move {
                        instances
                            .new(name, "vanilla".to_string(), version)
                            .map_err(|e| e.to_string())
                    },
                    Message::CreatedInstance,
                );
            }
            Message::CreatedInstance(Ok(_)) => {
                self.name = String::new();
                self.selected_version = None;
            }
            Message::CreatedInstance(Err(err)) => {
                eprintln!("Error: {}", err);
            }
        }

        ret
    }

    fn view(&self) -> Element<Message> {
        let title = text("Vanilla Installer").size(30);

        let name_text = text("Instance name");
        let name = text_input("", &self.name).on_input(Message::ChangeName);
        let choose_name = column![name_text, name].spacing(10).padding(10);
        let choose_name = container(choose_name)
            .width(Length::Fill)
            .style(style::card());

        let version_text = text("Select version");
        let version_picker = pick_list(
            &self.versions,
            self.selected_version.clone(),
            Message::SelectVersion,
        );
        let select_version = column![version_text, version_picker]
            .spacing(10)
            .padding(10);
        let select_version = container(select_version)
            .width(Length::Fill)
            .style(style::card());

        let create_button = button("Create")
            .style(style::circle_button())
            .padding(10)
            .on_press(Message::Create(None));
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
}
