// SPDX-FileCopyrightText: 2023 Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use iced::{
    widget::{button, column, container, image, scrollable, text, Image},
    Alignment, Element, Length,
};
use iced_aw::{FloatingElement, Wrap};

use crate::{
    components::{assets, icons},
    style, util, Message,
};

pub fn view(instances: &util::instances::Instances) -> Element<Message> {
    let mut wrap = Wrap::new();
    for instance in instances {
        let logo_handle = image::Handle::from_memory(assets::LOGO_PNG);
        let logo = Image::new(logo_handle).height(100);

        let c = container(
            column![
                logo,
                text(instance.to_owned()).size(20),
                button("Edit").style(style::circle_button()),
                button("Launch").style(style::circle_button()),
            ]
            .align_items(Alignment::Center)
            .spacing(10)
            .padding(10),
        )
        .style(style::card());
        wrap = wrap.push(container(c).padding(5));
    }

    let content = FloatingElement::new(scrollable(wrap).width(Length::Fill), || {
        container(button(icons::plus()).style(style::circle_button()))
            .padding([0, 20, 10, 0])
            .into()
    });

    column![text("Instances").size(30), content]
        .spacing(10)
        .padding(10)
        .into()
}
