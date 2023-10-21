// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuelquarneti@protonmail.com>
// SPDX-License-Identifier: GPL-3.0-only

use iced::{Alignment, Element, Length, theme, widget::{button, Column, container, scrollable, text, vertical_space}};
use iced::widget::Row;
use iced_aw::Wrap;

use crate::{assets, components::icons, Message, pages::{no_instances, Page}, style, util::instances::Instance};

pub fn view(instances: &Vec<Instance>) -> Element<Message> {
    if instances.is_empty() {
        return no_instances::view();
    }

    let mut wrap = Wrap::new().spacing(10.);
    for instance in instances {
        let logo = icons::view_png(assets::GRASS_PNG, 64);

        let actions = Row::new()
            .push(button(icons::view(icons::PLAY_OUTLINE)).style(style::circle_button(theme::Button::Secondary)))
            .push(button(icons::view(icons::COG_OUTLINE)).style(style::circle_button(theme::Button::Secondary)))
            .push(button(icons::view(icons::DELETE_OUTLINE)).style(style::circle_button(theme::Button::Secondary)))
            .push(button(icons::view(icons::FOLDER_OPEN_OUTLINE)).style(style::circle_button(theme::Button::Secondary)))
            .spacing(5);

        let col =
            Column::new()
                .push(logo)
                .push(text(&instance.name))
                .push(vertical_space(5))
                .push(actions)
                .align_items(Alignment::Center)
                .spacing(5);

        wrap = wrap.push(container(col).padding(10).style(style::card()));
    }

    let content = scrollable(wrap).width(Length::Fill).height(Length::Fill);

    Column::new().push(text("Instances").size(30)).push(content)
        .spacing(10)
        .padding(10)
        .into()
}
