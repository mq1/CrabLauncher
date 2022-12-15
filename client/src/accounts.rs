// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use anyhow::Result;
use iced::{
    theme,
    widget::{button, column, container, horizontal_space, radio, row, text},
    Element, Length,
};
use mclib::accounts::AccountsDocument;

use crate::{icons, style, Message};

pub fn view(accounts_doc: &Result<AccountsDocument>) -> Element<Message> {
    let heading = row![icons::manage_accounts().size(50), text("Accounts").size(50)].spacing(5);

    let accounts: Element<_> = match accounts_doc {
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
                            button(row![text("Remove"), icons::delete()].spacing(5))
                                .on_press(Message::RemoveAccount(account.clone()))
                                .style(theme::Button::Destructive),
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
        button(row![icons::person_add(), text("Add Account")].spacing(5))
            .on_press(Message::AddAccount),
    ]
    .spacing(20)
    .padding(20)
    .into()
}
