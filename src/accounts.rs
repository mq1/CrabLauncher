// SPDX-FileCopyrightText: 2023 Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use iced::{
    widget::{button, column, container, row, scrollable, text},
    Element, Length,
};
use iced_aw::FloatingElement;

use crate::{components::icons, style, util, Message};

pub fn view(accounts: &util::accounts::Accounts) -> Element<Message> {
    let mut content = column![];

    if let Some(active_account) = &accounts.active {
        let row = row![
            text("Active account:"),
            text(active_account.mc_username.to_owned())
        ]
        .spacing(5);

        content = content.push(row);
    }

    for account in &accounts.others {
        let row = row![text(account.mc_username.to_owned()), button("Select")].spacing(5);

        content = content.push(row);
    }

    let content = scrollable(content).width(Length::Fill).height(Length::Fill);

    let content = FloatingElement::new(content, || {
        container(
            button(icons::plus())
                .style(style::circle_button())
                .on_press(Message::AddAccount),
        )
        .padding([0, 20, 20, 0])
        .into()
    });

    column![text("Accounts").size(30), content]
        .spacing(10)
        .padding(10)
        .into()
}
