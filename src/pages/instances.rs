// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuq01@pm.me>
// SPDX-License-Identifier: GPL-3.0-only

use iced::{
    Alignment,
    Element, Length, widget::{button, Column, container, scrollable, text, vertical_space},
};
use iced_aw::Wrap;

use crate::{assets, components::icons, Message, pages::{no_instances, Page}, util::instances::Instance};

pub fn view(instances: &Vec<Instance>) -> Element<Message> {
    if instances.is_empty() {
        return no_instances::view();
    }

    let mut wrap = Wrap::new().spacing(10.);
    for (i, instance) in instances.iter().enumerate() {
        let logo = icons::view_png(assets::GRASS_PNG, 64);

        let open_instance = button(
            Column::new()
                .push(vertical_space(Length::Fill))
                .push(logo)
                .push(text(&instance.name))
                .push(vertical_space(Length::Fill))
                .align_items(Alignment::Center)
                .spacing(5)
        )
            .width(128)
            .height(160)
            .on_press(Message::ChangePage(Page::Instance(i)));

        wrap = wrap.push(container(open_instance));
    }

    let content = scrollable(wrap).width(Length::Fill).height(Length::Fill);

    Column::new().push(text("Instances").size(30)).push(content)
        .spacing(10)
        .padding(10)
        .into()
}
