// SPDX-FileCopyrightText: 2023 Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use iced::{
    widget::{button, column, container, row, scrollable, text, vertical_space},
    Element, Length,
};
use iced_aw::FloatingElement;

use crate::{icons, style, util, Message, View};

pub fn view(accounts: &util::accounts::Accounts) -> Element<Message> {
    let header = row![
        button(icons::arrow_left())
            .style(style::transparent_button())
            .on_press(Message::ChangeView(View::Instances)),
        text("Accounts").size(30)
    ]
    .spacing(5);

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

    let content = FloatingElement::new(scrollable(content).width(Length::Fill), || {
        container(button(icons::plus()).style(style::circle_button()))
            .padding([0, 20, 10, 0])
            .into()
    });

    column![header, content, vertical_space(Length::Fill),]
        .spacing(10)
        .padding(10)
        .into()
}
