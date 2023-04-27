// SPDX-FileCopyrightText: 2023 Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use std::{fs, path::PathBuf};

use anyhow::Result;
use iced::{
    widget::{button, column, container, horizontal_space, row, text, toggler, vertical_space},
    Element, Length,
};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use crate::{icons, style, Message, View, BASE_DIR};

pub static PATH: Lazy<PathBuf> = Lazy::new(|| BASE_DIR.join("settings.toml"));

#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub check_for_updates: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            check_for_updates: true,
        }
    }
}

impl Settings {
    pub fn load() -> Result<Self> {
        if !PATH.exists() {
            return Ok(Self::default());
        }

        let settings = fs::read_to_string(&*PATH)?;
        let settings: Self = toml::from_str(&settings)?;
        Ok(settings)
    }

    pub fn save(&self) -> Result<()> {
        let settings = toml::to_string_pretty(self)?;
        fs::write(&*PATH, settings)?;
        Ok(())
    }

    pub fn view(&self) -> Element<Message> {
        let check_for_updates = toggler(
            "Check for updates".to_owned(),
            self.check_for_updates,
            Message::CheckForUpdates,
        );

        let header = row![
            text("Settings").size(30),
            horizontal_space(Length::Fill),
            button(row![icons::arrow_left(), " Back"])
                .on_press(Message::ChangeView(View::Instances))
        ];

        let save_button =
            button(row!["Save ", icons::content_save()]).on_press(Message::SaveSettings);

        column![
            header,
            container(column![check_for_updates].padding(10)).style(style::card()),
            vertical_space(Length::Fill),
            row![horizontal_space(Length::Fill), save_button]
        ]
        .spacing(10)
        .padding(10)
        .into()
    }
}
