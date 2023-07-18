// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuq01@pm.me>
// SPDX-License-Identifier: GPL-3.0-only

use iced::{
    widget::{button, column, container, image, scrollable, text, Image},
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

        let mut wrap = Wrap::new();
        for instance in &self.list {
            let logo = icons::view_png(icons::LOGO_PNG, 64);

            let c = button(
                column![logo, text(&instance.name).size(20)]
                    .align_items(Alignment::Center)
                    .spacing(10)
                    .padding(10),
            )
            .on_press(Message::ChangeView(View::Instance(Some(
                instance.to_owned(),
            ))));
            wrap = wrap.push(container(c).padding(5));
        }

        let content = scrollable(wrap).width(Length::Fill).height(Length::Fill);

        column![text("Instances").size(30), content]
            .spacing(10)
            .padding(10)
            .into()
    }
}
