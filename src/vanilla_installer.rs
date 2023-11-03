// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuelquarneti@protonmail.com>
// SPDX-License-Identifier: GPL-3.0-only

use iced::widget::{
    button, horizontal_space, radio, scrollable, text, text_input, vertical_space, Column, Row,
};
use iced::{Element, Length};
use iced_aw::{card, CardStyles};

use crate::message::Message;
use crate::version_manifest::{Version, VersionManifest};

pub struct VanillaInstaller {
    pub version_manifest: Option<VersionManifest>,
    pub selected_version: Option<usize>,
    pub name: String,
}

impl VanillaInstaller {
    pub fn new() -> Self {
        Self {
            version_manifest: None,
            selected_version: None,
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

        let versions_chooser: Element<_> = match &self.version_manifest {
            None => text("Loading...").into(),
            Some(version_manifest) => {
                let mut version_picker = Column::new().spacing(4);

                for (i, version) in version_manifest.versions.iter().enumerate() {
                    let version_radio = radio(
                        version.to_string(),
                        i,
                        self.selected_version,
                        Message::ChangeVanillaInstallerVersion,
                    );

                    version_picker = version_picker.push(version_radio);
                }

                let version_picker = scrollable(version_picker).width(Length::Fill);

                version_picker.into()
            }
        };

        let versions_chooser = card(text("Choose a version"), versions_chooser)
            .height(Length::Fill)
            .style(CardStyles::Secondary);

        let create_button = button("Create").padding(10);
        let footer = Row::new()
            .push(horizontal_space(Length::Fill))
            .push(create_button);

        Column::new()
            .push(title)
            .push(name_chooser)
            .push(versions_chooser)
            .push(footer)
            .push(vertical_space(24))
            .padding(16)
            .spacing(16)
            .into()
    }
}
