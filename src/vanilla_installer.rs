// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuelquarneti@protonmail.com>
// SPDX-License-Identifier: GPL-3.0-only

use crate::message::Message;
use crate::style;
use iced::widget::{container, text, text_input, Column};
use iced::{Element, Length};
use iced_aw::{card, CardStyles};

pub struct VanillaInstaller {
    pub available_versions: Vec<String>,
    pub selected_version: String,
    pub name: String,
}

impl VanillaInstaller {
    pub fn new() -> Self {
        Self {
            available_versions: Vec::new(),
            selected_version: "".to_string(),
            name: "My new Instance".to_string(),
        }
    }

    pub fn view(&self) -> Element<Message> {
        let title = text("Vanilla Installer").size(30);

        let name_chooser = card(
            text("Instance name"),
            text_input("", &self.name).on_input(Message::ChangeVanillaInstallerName),
        )
        .style(CardStyles::Secondary);

        Column::new()
            .push(title)
            .push(name_chooser)
            .padding(16)
            .spacing(16)
            .into()
    }
}
