// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuelquarneti@protonmail.com>
// SPDX-License-Identifier: GPL-3.0-only

use std::fs;
use std::sync::Arc;

use anyhow::Result;
use iced::widget::{
    button, horizontal_space, image, scrollable, text, vertical_space, Column, Row,
};
use iced::{theme, Alignment, Length};
use iced_aw::{card, CardStyles, Wrap};
use rfd::{MessageButtons, MessageDialog, MessageDialogResult, MessageLevel};
use serde::{Deserialize, Serialize};

use crate::icon::Icon;
use crate::info::LOGO_PNG;
use crate::{style, BASE_DIR};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Instance {
    pub name: String,
    pub minecraft_version: String,
}

pub struct Instances {
    list: Vec<Instance>,
}

impl Instances {
    pub fn load() -> Result<Self> {
        let instances_dir = BASE_DIR.join("instances");

        let mut list = Vec::new();

        for instance_dir in fs::read_dir(instances_dir)? {
            let instance_dir = instance_dir?;
            println!("Found dir {:?}", instance_dir.path());

            // skip non directories and directories that don't contain an instance.toml file
            if !instance_dir.file_type()?.is_dir()
                || !instance_dir.path().join("instance.toml").exists()
            {
                continue;
            }

            let instance = fs::read_to_string(instance_dir.path().join("instance.toml"))?;
            let instance: Instance = toml::from_str(&instance)?;

            list.push(instance);
        }

        Ok(Self { list })
    }

    pub fn view(&self) -> iced::Element<'_, crate::Message> {
        if self.list.is_empty() {
            return Column::new()
                .push(vertical_space(48))
                .push(
                    Row::new()
                        .push(Icon::ArrowLeft.view(24))
                        .push(text("You don't have any instances yet. Create one!").size(25))
                        .align_items(Alignment::Center)
                        .spacing(8),
                )
                .padding(8)
                .into();
        }

        let mut wrap = Wrap::new().spacing(10.);
        for instance in &self.list {
            let logo = image::Handle::from_memory(LOGO_PNG);
            let logo = image(logo).width(100).height(100);

            let actions = Row::new()
                .push(horizontal_space(Length::Fill))
                .push(
                    button(Icon::PlayOutline.view(24))
                        .style(style::circle_button(theme::Button::Primary)),
                )
                .push(
                    button(Icon::CogOutline.view(24))
                        .style(style::circle_button(theme::Button::Primary)),
                )
                .push(
                    button(Icon::DeleteOutline.view(24))
                        .style(style::circle_button(theme::Button::Primary))
                        .on_press(crate::Message::DeleteInstance(instance.name.clone())),
                )
                .push(
                    button(Icon::FolderOpenOutline.view(24))
                        .style(style::circle_button(theme::Button::Primary))
                        .on_press(crate::Message::OpenInstanceFolder(instance.name.clone())),
                )
                .push(horizontal_space(Length::Fill))
                .spacing(5);

            let card = card(logo, text(&instance.name))
                .foot(actions)
                .style(CardStyles::Secondary)
                .width(Length::Fixed(200.));

            wrap = wrap.push(card);
        }

        let content = scrollable(wrap).width(Length::Fill).height(Length::Fill);

        Column::new()
            .push(text("Instances").size(30))
            .push(content)
            .spacing(8)
            .padding(8)
            .into()
    }

    pub fn create(&mut self, name: &str, minecraft_version: &str) -> Result<()> {
        let dir = BASE_DIR.join("instances").join(name);
        fs::create_dir_all(&dir)?;

        let instance = Instance {
            name: name.to_string(),
            minecraft_version: minecraft_version.to_string(),
        };

        let text = toml::to_string_pretty(&instance)?;
        fs::write(dir.join("instance.toml"), text)?;

        // add instance to list
        self.list.push(instance);

        Ok(())
    }

    pub fn open_instance_dir(&self, name: &str) -> Result<()> {
        let path = BASE_DIR.join("instances").join(name);
        open::that(path)?;
        Ok(())
    }

    pub fn delete_instance(&mut self, name: &str) -> Result<()> {
        let result = MessageDialog::new()
            .set_level(MessageLevel::Warning)
            .set_title("Delete instance")
            .set_description(format!(
                "Are you sure you want to delete the instance \"{}\"?",
                name
            ))
            .set_buttons(MessageButtons::OkCancel)
            .show();

        if result == MessageDialogResult::Cancel {
            return Ok(());
        }

        let path = BASE_DIR.join("instances").join(name);
        fs::remove_dir_all(path)?;

        // remove instance from list
        self.list.retain(|i| i.name != name);

        Ok(())
    }
}
