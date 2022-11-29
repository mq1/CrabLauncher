// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use anyhow::Result;
use iced::{
    widget::{
        button, column, container, horizontal_space, row, text, text_input, toggler,
        vertical_space, Column,
    },
    Element, Length,
};

use crate::{style, Message};

pub struct Settings {
    pub config: Result<mclib::launcher_config::LauncherConfig>,
}

impl Settings {
    pub fn new() -> Self {
        Self {
            config: mclib::launcher_config::read(),
        }
    }

    pub fn refresh(&mut self) {
        self.config = mclib::launcher_config::read();
    }

    pub fn view(&self) -> Element<Message> {
        let heading = text("Settings").size(50);

        let settings: Element<_> = match &self.config {
            Ok(config) => {
                let mut settings = Column::new().spacing(10);

                if cfg!(feature = "check-for-updates") {
                    settings = settings.push(
                        container(toggler(
                            "Automatically check for updates".to_string(),
                            config.automatically_check_for_updates,
                            Message::UpdatesTogglerChanged,
                        ))
                        .padding(10)
                        .style(style::card()),
                    );
                }

                settings = settings.push(
                    container(toggler(
                        "Automatically update JVM".to_string(),
                        config.automatically_update_jvm,
                        Message::UpdateJvmTogglerChanged,
                    ))
                    .padding(10)
                    .style(style::card()),
                );

                settings = settings.push(
                    container(toggler(
                        "Automatically optimize JVM".to_string(),
                        config.automatically_optimize_jvm_arguments,
                        Message::OptimizeJvmTogglerChanged,
                    ))
                    .padding(10)
                    .style(style::card()),
                );

                settings = settings.push(
                    container(row![
                        text("JVM memory"),
                        horizontal_space(Length::Fill),
                        text_input("JVM memory", &config.jvm_memory, Message::UpdateJvmMemory),
                    ])
                    .padding(10)
                    .style(style::card()),
                );

                settings.into()
            }
            Err(_) => text("Failed to load settings").into(),
        };

        let footer = row![
            horizontal_space(Length::Fill),
            button("Reset to default settings").on_press(Message::ResetConfig),
            button("Save settings").on_press(Message::SaveConfig),
        ]
        .spacing(10);

        column![heading, settings, vertical_space(Length::Fill), footer]
            .spacing(20)
            .padding(20)
            .into()
    }
}
