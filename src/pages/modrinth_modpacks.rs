// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuq01@pm.me>
// SPDX-License-Identifier: GPL-3.0-only

use iced::{Alignment, Element, Length, widget::{button, Column, horizontal_space, Row, scrollable, text}};

use crate::components::icons;
use crate::types::messages::Message;
use crate::types::modrinth_modpacks::ModrinthModpacks;

pub fn view(modrinth_modpacks: &ModrinthModpacks) -> Element<Message> {
    let title = text("Modrinth Modpacks").size(30);

    let mut list = Column::new().spacing(10).padding([0, 20, 0, 0]);
    for project in &modrinth_modpacks.projects {
        let mut info = Row::new().align_items(Alignment::Center)
            .padding(5)
            .spacing(5)
            .push(text(project.title.to_owned()));

        if !project.display_categories.is_empty() {
            let categories = format!("[{}]", project.display_categories.join(","));

            info = info.push(text(categories));
        }

        info = info
            .push(horizontal_space(Length::Fill))
            .push(icons::view(icons::DOWNLOAD_OUTLINE))
            .push(text(format!("{} Downloads", project.downloads)));

        let button = button(info);

        list = list.push(button);
    }

    let scrollable = scrollable(list).height(Length::Fill);

    Column::new().push(title).push(scrollable).spacing(10).padding(10).into()
}
