// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use iced::{
    widget::{button, column, container, horizontal_space, row, scrollable, text, Column},
    Element, Length,
};

use crate::{style, InstallerInfo, Message};

pub fn view(info: &InstallerInfo) -> Element<Message> {
    let heading = text("Modrinth Modpacks").size(50);

    let content: Element<_> = match &info.available_modpacks {
        Some(Ok(modpacks)) => {
            let mut column = Column::new().spacing(10);
            for modpack in &modpacks.hits {
                let version_text = text(format!("[Latest Version: {}]", modpack.latest_version));

                let select_button = button("Select").on_press(Message::ModpackSelected(modpack.clone()));

                let row = row![
                    text(&modpack.title),
                    version_text,
                    horizontal_space(Length::Fill),
                    select_button
                ]
                .spacing(10)
                .padding(10);

                let container = container(row).style(style::card());

                column = column.push(container);
            }

            scrollable(column).into()
        }
        Some(Err(error)) => text(error.to_string()).into(),
        None => text("Loading...").into(),
    };

    column![heading, content].spacing(10).padding(20).into()
}
