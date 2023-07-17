// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuq01@pm.me>
// SPDX-License-Identifier: GPL-3.0-only

use iced::{
    widget::{button, column, text, vertical_space, Button},
    Alignment, Element, Length,
};
use iced_aw::Wrap;

use crate::{components::icons, pages::Page, Message, View};

fn btn(
    name: &str,
    installer_view: View,
    icon: Element<'static, Message>,
) -> Button<'static, Message> {
    let content = column![
        vertical_space(Length::Fill),
        icon,
        text(name),
        vertical_space(Length::Fill),
    ]
    .align_items(Alignment::Center)
    .spacing(5);

    button(content)
        .height(128)
        .width(128)
        .on_press(Message::ChangeView(installer_view))
}

pub struct NewInstance;

impl Page for NewInstance {
    type Message = Message;

    fn update(&mut self, _message: Message) -> iced::Command<Message> {
        iced::Command::none()
    }

    fn view(&self) -> Element<'static, Message> {
        let title = text("New instance").size(30);

        let mut wrap = Wrap::new().spacing(10.);

        // Vanilla
        let vanilla_btn = btn(
            "Vanilla",
            View::VanillaInstaller,
            icons::view_png(icons::GRASS_PNG),
        );
        wrap = wrap.push(vanilla_btn);

        // Modrinth
        let modrinth_btn = btn(
            "Modrinth",
            View::ModrinthModpacks,
            icons::view_custom(icons::MODRINTH, 64),
        );
        wrap = wrap.push(modrinth_btn);

        column![title, wrap].spacing(10).padding(10).into()
    }
}
