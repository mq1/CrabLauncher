// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuq01@pm.me>
// SPDX-License-Identifier: GPL-3.0-only

use iced::{
    widget::{button, column, container, image, scrollable, text, Image, vertical_space},
    Alignment, Command, Element, Length,
};
use iced_aw::Wrap;

use crate::{
    components::icons, pages::no_instances::NoInstances, pages::Page, util::instances::Instances,
    Message, View,
};

impl Page for Instances {
    type Message = Message;

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        if self.list.is_empty() {
            return NoInstances.view();
        }

        let mut wrap = Wrap::new().spacing(10.);
        for instance in &self.list {
            let logo = icons::view_png(icons::GRASS_PNG, 64);

            let c = button(
                column![
                    vertical_space(Length::Fill),
                    logo,
                    text(&instance.name).size(20),
                    vertical_space(Length::Fill),
                ]
                .align_items(Alignment::Center)
                .spacing(5),
            )
                .width(128)
                .height(128)
            .on_press(Message::ChangeView(View::Instance(Some(
                instance.to_owned(),
            ))));
            wrap = wrap.push(container(c));
        }

        let content = scrollable(wrap).width(Length::Fill).height(Length::Fill);

        column![text("Instances").size(30), content]
            .spacing(10)
            .padding(10)
            .into()
    }
}
