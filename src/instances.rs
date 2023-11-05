// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuelquarneti@protonmail.com>
// SPDX-License-Identifier: GPL-3.0-only

use std::fs;
use std::sync::Arc;

use iced::widget::{text, vertical_space, Column, Row};
use iced::Alignment;
use serde::Serialize;

use crate::icon::Icon;
use crate::BASE_DIR;

#[derive(Serialize)]
pub struct Instance {
    pub name: String,
    pub minecraft_version: String,
}

pub struct Instances {
    list: Vec<Instance>,
}

impl Instances {
    pub fn new() -> Self {
        Self { list: Vec::new() }
    }

    pub fn view(&self) -> iced::Element<'_, crate::Message> {
        Column::new()
            .push(vertical_space(48))
            .push(
                Row::new()
                    .push(Icon::ArrowLeft.view(24))
                    .push(text("You don't have any instances yet. Create one!").size(25))
                    .align_items(Alignment::Center)
                    .spacing(8),
            )
            .padding(8)
            .into()
    }

    fn _create(name: &str, minecraft_version: &str) -> Result<(), anyhow::Error> {
        let dir = BASE_DIR.join("instances").join(name);
        fs::create_dir_all(&dir)?;

        let instance = Instance {
            name: name.to_string(),
            minecraft_version: minecraft_version.to_string(),
        };

        let text = toml::to_string_pretty(&instance)?;
        fs::write(dir.join("instance.toml"), text)?;

        Ok(())
    }

    pub fn create(&self, name: &str, minecraft_version: &str) -> Result<(), Arc<anyhow::Error>> {
        Self::_create(name, minecraft_version).map_err(Arc::new)
    }
}
