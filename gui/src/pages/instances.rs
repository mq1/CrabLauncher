// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuelquarneti@protonmail.com>
// SPDX-License-Identifier: GPL-3.0-only

use iced::widget::{button, horizontal_space, scrollable, text, Column, Row};
use iced::{theme, Element, Length};
use iced_aw::helpers::card;
use iced_aw::{CardStyles, Wrap};
use lib::instances::Instances;

use crate::{assets, components::icons, pages::no_instances, style, Message};

pub fn view(instances: &Instances) -> Element<Message> {
    if instances.list.is_empty() {
        return no_instances::view();
    }

    let mut wrap = Wrap::new().spacing(10.);
    for (name, _) in &instances.list {
        let logo = icons::view_png(assets::GRASS_PNG, 100);

        let actions = Row::new()
            .push(horizontal_space(Length::Fill))
            .push(
                button(icons::view(icons::PLAY_OUTLINE))
                    .on_press(Message::LaunchInstance(name.clone()))
                    .style(style::circle_button(theme::Button::Secondary)),
            )
            .push(
                button(icons::view(icons::COG_OUTLINE))
                    .on_press(Message::OpenInstanceConfig(name.clone()))
                    .style(style::circle_button(theme::Button::Secondary)),
            )
            .push(
                button(icons::view(icons::DELETE_OUTLINE))
                    .on_press(Message::DeleteInstance(name.clone()))
                    .style(style::circle_button(theme::Button::Secondary)),
            )
            .push(
                button(icons::view(icons::FOLDER_OPEN_OUTLINE))
                    .on_press(Message::OpenInstanceFolder(name.clone()))
                    .style(style::circle_button(theme::Button::Secondary)),
            )
            .push(horizontal_space(Length::Fill))
            .spacing(5);

        let card = card(logo, text(name))
            .foot(actions)
            .style(CardStyles::Secondary)
            .width(Length::Fixed(200.));

        wrap = wrap.push(card);
    }

    let content = scrollable(wrap).width(Length::Fill).height(Length::Fill);

    Column::new()
        .push(text("Instances").size(30))
        .push(content)
        .spacing(10)
        .padding(10)
        .into()
}
