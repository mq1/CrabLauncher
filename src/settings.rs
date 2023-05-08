// SPDX-FileCopyrightText: 2023 Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use iced::{
    widget::{button, column, container, horizontal_space, row, text, toggler, vertical_space},
    Element, Length,
};

use crate::{icons, style, util, Message, View};

pub fn view(settings: &util::settings::Settings) -> Element<Message> {
    let check_for_updates = toggler(
        "Check for updates".to_owned(),
        settings.check_for_updates,
        Message::CheckForUpdates,
    );

    let header = row![
        button(icons::arrow_left())
            .style(style::transparent_button())
            .on_press(Message::ChangeView(View::Instances)),
        text("Settings").size(30)
    ]
    .spacing(5);

    let save_button = button(icons::content_save())
        .style(style::circle_button())
        .on_press(Message::SaveSettings);

    column![
        header,
        container(column![check_for_updates].padding(10)).style(style::card()),
        vertical_space(Length::Fill),
        row![horizontal_space(Length::Fill), save_button]
    ]
    .spacing(10)
    .padding(10)
    .into()
}
