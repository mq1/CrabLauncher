// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuelquarneti@protonmail.com>
// SPDX-License-Identifier: GPL-3.0-only

use std::fs;

use anyhow::Result;
use iced::widget::{
    button, horizontal_space, text, text_input, toggler, vertical_space, Column, Row,
};
use iced::{theme, Alignment, Element, Length};
use iced_aw::{card, CardStyles};
use serde::{Deserialize, Serialize};

use crate::icon::Icon;
use crate::message::Message;
use crate::{style, BASE_DIR};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Settings {
    pub auto_update_check: bool,
    pub java_path: String,
    pub java_memory: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            auto_update_check: true,
            java_path: "java".to_string(),
            java_memory: "4G".to_string(),
        }
    }
}

impl Settings {
    pub fn load() -> Self {
        let path = BASE_DIR.join("settings.toml");
        let data = fs::read_to_string(path).unwrap_or_default();

        toml::from_str(&data).unwrap_or_default()
    }

    pub fn save(&self) -> Result<()> {
        let path = BASE_DIR.join("settings.toml");
        let data = toml::to_string_pretty(self)?;

        fs::write(path, data)?;

        Ok(())
    }

    pub fn view(&self) -> Element<Message> {
        // -----------------------------------------------------------------------------------------

        let mut col = Column::new();

        #[cfg(feature = "updater")]
        {
            let check_for_updates = toggler(
                "Automatically check for updates".to_owned(),
                self.auto_update_check,
                Message::SetAutoUpdateCheck,
            );

            col = col.push(check_for_updates);
        }

        let launcher_settings = card(text("Launcher Settings"), col).style(CardStyles::Secondary);

        // -----------------------------------------------------------------------------------------

        let mut col = Column::new();

        // java path text input
        col = col.push(text("Java path"));
        let java_path = text_input("Java path", &self.java_path).on_input(Message::ChangeJavaPath);
        col = col.push(java_path);

        col = col.push(vertical_space(16));

        // java memory text input
        col = col.push(text("Java memory"));
        let java_memory =
            text_input("Java memory", &self.java_memory).on_input(Message::ChangeJavaMemory);
        col = col.push(java_memory);

        let java_settings = card(text("Java Settings"), col).style(CardStyles::Secondary);

        // -----------------------------------------------------------------------------------------

        let save_button = button(
            Row::new()
                .push(text(" Save "))
                .push(Icon::ContentSaveOutline.view(24))
                .padding(5)
                .align_items(Alignment::Center),
        )
        .style(style::circle_button(theme::Button::Positive))
        .on_press(Message::SaveSettings);

        Column::new()
            .push(text("Settings").size(30))
            .push(launcher_settings)
            .push(java_settings)
            .push(vertical_space(Length::Fill))
            .push(
                Row::new()
                    .push(horizontal_space(Length::Fill))
                    .push(save_button),
            )
            .spacing(8)
            .padding(8)
            .into()
    }
}
