// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuelquarneti@protonmail.com>
// SPDX-License-Identifier: GPL-3.0-only

use crate::components::icon::Icon;
use iced::{
    theme,
    widget::{button, container, horizontal_space, scrollable, text, vertical_space, Column, Row},
    Alignment, Element, Length,
};
use iced_aw::floating_element;
use lib::accounts::Accounts;

use crate::pages::Page;
use crate::style;
use crate::types::messages::Message;

pub fn view(accounts: &Accounts) -> Element<Message> {
    let mut content = Column::new()
        .width(Length::Fill)
        .height(Length::Fill)
        .spacing(10);

    if let Some(active_account) = &accounts.active {
        let row = Row::new()
            .push(text(&active_account.mc_username))
            .push(horizontal_space(Length::Fill))
            .push(
                button(Icon::DeleteOutline.view(24))
                    .on_press(Message::RemoveAccount(active_account.clone()))
                    .style(style::circle_button(theme::Button::Destructive)),
            )
            .align_items(Alignment::Center)
            .padding(10);

        let active = container(row).style(style::card());
        content = content
            .push(text("Active account"))
            .push(active)
            .push(vertical_space(10));
    }

    if !accounts.others.is_empty() {
        let mut others = Column::new().spacing(10);

        for account in &accounts.others {
            let row = Row::new()
                .push(text(&account.mc_username))
                .push(horizontal_space(Length::Fill))
                .push(
                    button(Icon::AccountCheckOutline.view(24))
                        .on_press(Message::SelectAccount(account.clone()))
                        .style(style::circle_button(theme::Button::Positive)),
                )
                .push(
                    button(Icon::DeleteOutline.view(24))
                        .on_press(Message::RemoveAccount(account.clone()))
                        .style(style::circle_button(theme::Button::Destructive)),
                )
                .align_items(Alignment::Center)
                .padding(10)
                .spacing(5);

            others = others.push(container(row).style(style::card()));
        }

        content = content
            .push(text("Other accounts"))
            .push(scrollable(others));
    }

    let content = floating_element(content, {
        let mut row = Row::new()
            .spacing(10)
            .align_items(Alignment::Center)
            .padding([0, 20, 20, 0]);

        #[cfg(feature = "offline-accounts")]
        {
            let add_offline_account_button = button(
                Row::new()
                    .push(text(" Add offline account "))
                    .push(Icon::AccountPlusOutline.view(24))
                    .align_items(Alignment::Center)
                    .padding(5),
            )
            .on_press(Message::ChangePage(Page::AddingOfflineAccount))
            .style(style::circle_button(theme::Button::Secondary));

            row = row.push(add_offline_account_button);
        }

        let add_account_button = button(
            Row::new()
                .push(text(" Add account "))
                .push(Icon::AccountPlusOutline.view(24))
                .align_items(Alignment::Center)
                .padding(5),
        )
        .on_press(Message::AddAccount)
        .style(style::circle_button(theme::Button::Primary));

        row.push(add_account_button)
    });

    Column::new()
        .push(text("Accounts").size(30))
        .push(content)
        .spacing(10)
        .padding(10)
        .into()
}
