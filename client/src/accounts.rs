// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use anyhow::Result;
use iced::{
    widget::{button, column, container, horizontal_space, radio, row, text},
    Command, Element, Length,
};
use mclib::msa::{Account, AccountId};
use native_dialog::{MessageDialog, MessageType};

use crate::style;

#[derive(Debug, Clone)]
pub enum Message {
    RefreshAccounts,
    RemoveAccount(Account),
    AddAccount,
    AccountAdded(Result<(), String>),
    AccountSelected(AccountId),
}

pub struct Accounts {
    pub document: Result<mclib::accounts::AccountsDocument>,
}

impl Accounts {
    pub fn new() -> Self {
        Self {
            document: mclib::accounts::read(),
        }
    }

    pub fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::RefreshAccounts => {
                self.document = mclib::accounts::read();
            }
            Message::RemoveAccount(account) => {
                let yes = MessageDialog::new()
                    .set_type(MessageType::Warning)
                    .set_title("Remove account")
                    .set_text(&format!(
                        "Are you sure you want to remove {}?",
                        account.mc_username
                    ))
                    .show_confirm()
                    .unwrap();

                if yes {
                    mclib::accounts::remove(account).unwrap();
                    self.update(Message::RefreshAccounts);
                }
            }
            Message::AccountSelected(account) => {
                mclib::accounts::set_active(account).unwrap();
                self.update(Message::RefreshAccounts);
            }
            Message::AddAccount => {
                return Command::perform(
                    async { mclib::accounts::add().map_err(|e| e.to_string()) },
                    Message::AccountAdded,
                );
            }
            Message::AccountAdded(res) => {
                if let Some(err) = res.err() {
                    MessageDialog::new()
                        .set_type(MessageType::Error)
                        .set_title("Error adding account")
                        .set_text(&err)
                        .show_alert()
                        .unwrap();
                }

                self.update(Message::RefreshAccounts);
            }
        }

        Command::none()
    }

    pub fn view(&self) -> Element<Message> {
        let heading = text("Accounts").size(50);

        let accounts: Element<_> = match &self.document {
            Ok(document) => column(
                document
                    .accounts
                    .iter()
                    .map(|account| {
                        container(
                            row![
                                radio(
                                    account.mc_username.to_owned(),
                                    account.mc_id,
                                    document.active_account,
                                    Message::AccountSelected
                                ),
                                horizontal_space(Length::Fill),
                                button("Remove").on_press(Message::RemoveAccount(account.clone())),
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
            Err(_) => text("Failed to load accounts").into(),
        };

        column![
            heading,
            accounts,
            button("Add account").on_press(Message::AddAccount),
        ]
        .spacing(20)
        .padding(20)
        .into()
    }
}
