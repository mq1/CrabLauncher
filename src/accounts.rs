// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use color_eyre::Result;
use iced::{
    widget::{button, column, container, horizontal_space, row, text, vertical_space, radio},
    Element, Length,
};

use crate::{
    lib::{self, accounts::AccountsDocument},
    style, Message,
};

pub struct AccountsView {
    pub document: Result<AccountsDocument>,
}

impl AccountsView {
    pub fn new() -> Self {
        Self {
            document: lib::accounts::read(),
        }
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
                                radio(account.mc_username.to_owned(), account.mc_id, document.active_account, Message::AccountSelected),
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

        column![heading, vertical_space(Length::Units(20)), accounts]
            .padding(20)
            .into()
    }
}
