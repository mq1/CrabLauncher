// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuelquarneti@protonmail.com>
// SPDX-License-Identifier: GPL-3.0-only

use iced::widget::{button, horizontal_space, image, scrollable, text, Column, Row};
use iced::{theme, Element, Length};
use iced_aw::helpers::card;
use iced_aw::{CardStyles, Wrap};
use lib::instances::Instances;

use crate::components::icon::Icon;
use crate::{pages::no_instances, style, Message, LOGO_PNG};

pub fn view(instances: &Instances) -> Element<Message> {
    if instances.list.is_empty() {
        return no_instances::view();
    }

    let mut wrap = Wrap::new().spacing(10.);
    for (name, _) in &instances.list {
        let logo = image::Handle::from_memory(LOGO_PNG);
        let logo = image(logo).width(100).height(100);

        let actions = Row::new()
            .push(horizontal_space(Length::Fill))
            .push(
                button(Icon::PlayOutline.view(24))
                    .on_press(Message::LaunchInstance(name.clone()))
                    .style(style::circle_button(theme::Button::Secondary)),
            )
            .push(
                button(Icon::CogOutline.view(24))
                    .on_press(Message::OpenInstanceConfig(name.clone()))
                    .style(style::circle_button(theme::Button::Secondary)),
            )
            .push(
                button(Icon::DeleteOutline.view(24))
                    .on_press(Message::DeleteInstance(name.clone()))
                    .style(style::circle_button(theme::Button::Secondary)),
            )
            .push(
                button(Icon::FolderOpenOutline.view(24))
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
