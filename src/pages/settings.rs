// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuq01@pm.me>
// SPDX-License-Identifier: GPL-3.0-only

use iced::{
    Alignment,
    Element, Length, theme, widget::{
        button, Column, container, horizontal_space, Row, text, toggler, vertical_space,
    },
};

use crate::{components::icons, style, util::settings::Settings};
use crate::types::messages::Message;

pub fn view(settings: &Settings) -> Element<Message> {
    let mut col = Column::new().padding(10);

    #[cfg(feature = "updater")]
    {
        let check_for_updates = toggler(
            "Automatically check for updates".to_owned(),
            settings.check_for_updates,
            Message::SetCheckForUpdates,
        );

        col = col.push(check_for_updates);
    }

    let save_button = button(
        Row::new().push(text(" Save ")).push(icons::view(icons::CONTENT_SAVE_OUTLINE))
            .padding(5)
            .align_items(Alignment::Center),
    )
        .style(style::circle_button(theme::Button::Positive))
        .on_press(Message::SaveSettings);

    Column::new()
        .push(text("Settings").size(30))
        .push(container(col).style(style::card()))
        .push(vertical_space(Length::Fill))
        .push(Row::new().push(horizontal_space(Length::Fill)).push(save_button))
        .spacing(10)
        .padding(10)
        .into()
}
