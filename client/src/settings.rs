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
use native_dialog::{MessageDialog, MessageType};

use crate::style;

#[derive(Debug, Clone)]
pub enum Message {
    UpdatesTogglerChanged(bool),
    UpdateJvmTogglerChanged(bool),
    OptimizeJvmTogglerChanged(bool),
    UpdateJvmMemory(String),
    ResetConfig,
    SaveConfig,
}

pub struct Settings {
    pub config: Result<mclib::launcher_config::LauncherConfig>,
}

impl Settings {
    pub fn new() -> Self {
        Self {
            config: mclib::launcher_config::read(),
        }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::UpdatesTogglerChanged(enabled) => {
                if let Ok(ref mut config) = self.config {
                    config.automatically_check_for_updates = enabled;
                }
            }
            Message::UpdateJvmTogglerChanged(enabled) => {
                if let Ok(ref mut config) = self.config {
                    config.automatically_update_jvm = enabled;
                }
            }
            Message::OptimizeJvmTogglerChanged(enabled) => {
                if let Ok(ref mut config) = self.config {
                    config.automatically_optimize_jvm_arguments = enabled;
                }
            }
            Message::UpdateJvmMemory(memory) => {
                if let Ok(ref mut config) = self.config {
                    config.jvm_memory = memory;
                }
            }
            Message::ResetConfig => {
                let yes = MessageDialog::new()
                    .set_type(MessageType::Warning)
                    .set_title("Reset config")
                    .set_text("Are you sure you want to reset the config?")
                    .show_confirm()
                    .unwrap();

                if yes {
                    self.config = mclib::launcher_config::reset();
                }
            }
            Message::SaveConfig => {
                if let Ok(ref config) = self.config {
                    if let Err(e) = mclib::launcher_config::write(config) {
                        MessageDialog::new()
                            .set_type(MessageType::Error)
                            .set_title("Error")
                            .set_text(&format!("Failed to save config: {e}"))
                            .show_alert()
                            .unwrap();
                    }
                }
            }
        }
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
