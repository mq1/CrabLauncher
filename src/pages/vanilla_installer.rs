// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuq01@pm.me>
// SPDX-License-Identifier: GPL-3.0-only

use iced::{
    Command,
    Element,
    Length, theme, widget::{
        button, Column, container, horizontal_space, radio, Row, scrollable, text,
        text_input, toggler,
    },
};

use crate::{pages::Page, style, types::generic_error::GenericError, util};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Message {
    GetVersions,
    GotVersions(Result<Vec<String>, GenericError>),
    ChangeName(String),
    SetOptimizeJvm(bool),
    SetMemory(String),
    SelectVersion(usize),
    Create,
}

pub struct VanillaInstaller {
    pub versions: Vec<String>,
    pub selected_version: Option<usize>,
    pub name: String,
    pub optimize_jvm: bool,
    pub memory: String,
}

impl VanillaInstaller {
    pub fn new() -> Self {
        Self {
            versions: Vec::new(),
            selected_version: None,
            name: String::new(),
            optimize_jvm: true,
            memory: "2G".to_string(),
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
                    util::vanilla_installer::get_versions(),
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
            Message::SetOptimizeJvm(optimize_jvm) => {
                self.optimize_jvm = optimize_jvm;
            }
            Message::SetMemory(memory) => {
                self.memory = memory;
            }
            Message::SelectVersion(version) => {
                self.selected_version = Some(version);
            }
            Message::Create => {
                self.name = String::new();
                self.selected_version = None;
            }
        }

        ret
    }

    fn view(&self) -> Element<Message> {
        let title = text("Vanilla Installer").size(30);

        let name_text = text("Instance name");
        let name = text_input("", &self.name).on_input(Message::ChangeName);
        let choose_name = Column::new().push(name_text).push(name).spacing(10).padding(10);
        let choose_name = container(choose_name)
            .width(Length::Fill)
            .style(style::card());

        let memory_text = text("Memory");
        let memory = text_input("", &self.memory).on_input(Message::SetMemory);
        let choose_memory = Column::new().push(memory_text).push(memory).spacing(10).padding(10);
        let choose_memory = container(choose_memory)
            .width(Length::Fill)
            .style(style::card());

        let optimize_jvm = toggler(
            "Optimize JVM".to_string(),
            self.optimize_jvm,
            Message::SetOptimizeJvm,
        );
        let optimize_jvm = container(optimize_jvm).padding(10);
        let optimize_jvm = container(optimize_jvm)
            .width(Length::Fill)
            .style(style::card());

        let version_text = text("Select version");
        let mut version_picker = Column::new().spacing(5);
        for (i, version) in self.versions.iter().enumerate() {
            version_picker = version_picker.push(radio(
                version.to_owned(),
                i,
                self.selected_version,
                Message::SelectVersion,
            ));
        }

        let version_picker = scrollable(version_picker).width(Length::Fill);

        let select_version = Column::new().push(version_text).push(version_picker)
            .spacing(10)
            .padding(10);
        let select_version = container(select_version)
            .height(Length::Fill)
            .style(style::card());

        let create_button = button("Create")
            .style(style::circle_button(theme::Button::Primary))
            .padding(10)
            .on_press(Message::Create);
        let footer = Row::new().push(horizontal_space(Length::Fill)).push(create_button);

        Column::new()
            .push(title)
            .push(choose_name)
            .push(choose_memory)
            .push(optimize_jvm)
            .push(select_version)
            .push(footer)
            .spacing(10)
            .padding(10)
            .into()
    }
}
