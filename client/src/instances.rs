// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use anyhow::Result;
use iced::{
    widget::{button, column, container, horizontal_space, row, text},
    Command, Element, Length,
};
use mclib::{accounts::AccountsDocument, instances::Instance};
use native_dialog::{MessageDialog, MessageType};

use crate::style;

#[derive(Debug, Clone)]
pub enum Message {
    RemoveInstance(String),
    LaunchInstance(Instance),
    NewInstance,
    RefreshInstances,
    InstanceClosed(Result<(), String>),
}

pub struct Instances {
    list: Result<Vec<mclib::instances::Instance>>,
}

impl Instances {
    pub fn new() -> Self {
        Self {
            list: mclib::instances::list(),
        }
    }

    pub fn update(
        &mut self,
        message: Message,
        accounts: &Result<AccountsDocument>,
    ) -> Command<Message> {
        match message {
            Message::RemoveInstance(name) => {
                let yes = MessageDialog::new()
                    .set_type(MessageType::Warning)
                    .set_title("Remove instance")
                    .set_text(&format!("Are you sure you want to remove {}?", &name))
                    .show_confirm()
                    .unwrap();

                if yes {
                    mclib::instances::remove(&name).unwrap();
                    self.update(Message::RefreshInstances, accounts);
                }
            }
            Message::LaunchInstance(instance) => {
                if let Ok(accounts) = accounts {
                    if !accounts.has_account_selected() {
                        MessageDialog::new()
                            .set_type(MessageType::Warning)
                            .set_title("No account selected")
                            .set_text("Please select an account to launch the game")
                            .show_alert()
                            .unwrap();

                        return Command::none();
                    }

                    return Command::perform(
                        async { mclib::instances::launch(instance).map_err(|e| e.to_string()) },
                        Message::InstanceClosed,
                    );
                }
            }
            Message::NewInstance => {}
            Message::RefreshInstances => {
                self.list = mclib::instances::list();
            }
            Message::InstanceClosed(res) => {
                if let Err(e) = res {
                    MessageDialog::new()
                        .set_type(MessageType::Error)
                        .set_title("Error")
                        .set_text(&e)
                        .show_alert()
                        .unwrap();
                }
            }
        }

        Command::none()
    }

    pub fn view(&self) -> Element<Message> {
        let heading = text("Instances").size(50);

        let instances_list: Element<_> = match &self.list {
            Ok(instances) => column(
                instances
                    .iter()
                    .map(|instance| {
                        container(
                            row![
                                text(format!(
                                    "{} [{}] [{}]",
                                    instance.name,
                                    instance.info.instance_type,
                                    instance.info.minecraft_version
                                )),
                                horizontal_space(Length::Fill),
                                button("Remove")
                                    .on_press(Message::RemoveInstance(instance.name.clone())),
                                button("Launch")
                                    .on_press(Message::LaunchInstance(instance.clone())),
                            ]
                            .spacing(10)
                            .padding(10),
                        )
                        .style(style::card())
                        .into()
                    })
                    .collect(),
            )
            .spacing(10)
            .into(),
            Err(_) => text("Failed to load instances").into(),
        };

        let new_instance_button = button("New instance").on_press(Message::NewInstance);

        column![heading, instances_list, new_instance_button]
            .spacing(20)
            .padding(20)
            .into()
    }
}
