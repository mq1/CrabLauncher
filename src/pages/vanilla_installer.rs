// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuq01@pm.me>
// SPDX-License-Identifier: GPL-3.0-only

use iced::{
    Element,
    Length, theme, widget::{
        button, Column, container, horizontal_space, radio, Row, scrollable, text,
        text_input, toggler,
    },
};

use crate::style;
use crate::types::messages::Message;
use crate::types::vanilla_installer::VanillaInstaller;

pub fn view(vanilla_installer: &VanillaInstaller) -> Element<Message> {
    let title = text("Vanilla Installer").size(30);

    let name_text = text("Instance name");
    let name = text_input("", &vanilla_installer.name).on_input(Message::ChangeName);
    let choose_name = Column::new().push(name_text).push(name).spacing(10).padding(10);
    let choose_name = container(choose_name)
        .width(Length::Fill)
        .style(style::card());

    let memory_text = text("Memory");
    let memory = text_input("", &vanilla_installer.memory).on_input(Message::SetMemory);
    let choose_memory = Column::new().push(memory_text).push(memory).spacing(10).padding(10);
    let choose_memory = container(choose_memory)
        .width(Length::Fill)
        .style(style::card());

    let optimize_jvm = toggler(
        "Optimize JVM".to_string(),
        vanilla_installer.optimize_jvm,
        Message::SetOptimizeJvm,
    );
    let optimize_jvm = container(optimize_jvm).padding(10);
    let optimize_jvm = container(optimize_jvm)
        .width(Length::Fill)
        .style(style::card());

    let version_text = text("Select version");
    let mut version_picker = Column::new().spacing(5);
    for (i, version) in vanilla_installer.versions.iter().enumerate() {
        version_picker = version_picker.push(radio(
            version.to_owned(),
            i,
            vanilla_installer.selected_version,
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
        .on_press(Message::CreateInstance);
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
