// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuelquarneti@protonmail.com>
// SPDX-License-Identifier: GPL-3.0-only

use iced::{
    widget::{button, text, vertical_space, Button, Column},
    Alignment, Element, Length,
};
use iced_aw::Wrap;

use crate::{components::icon::Icon, pages::Page, Message};

fn installer_button(
    name: &str,
    page: Page,
    icon: Element<'static, Message>,
) -> Button<'static, Message> {
    let content = Column::new()
        .push(vertical_space(Length::Fill))
        .push(icon)
        .push(text(name))
        .push(vertical_space(Length::Fill))
        .align_items(Alignment::Center)
        .spacing(5);

    button(content)
        .height(128)
        .width(128)
        .on_press(Message::ChangePage(page))
}

pub fn view() -> Element<'static, Message> {
    let title = text("New instance").size(30);

    let mut wrap = Wrap::new().spacing(10.);

    // Vanilla
    let vanilla_btn = installer_button("Vanilla", Page::VanillaInstaller, Icon::Minecraft.view(64));
    wrap = wrap.push(vanilla_btn);

    // Modrinth
    let modrinth_btn =
        installer_button("Modrinth", Page::ModrinthModpacks, Icon::Modrinth.view(64));
    wrap = wrap.push(modrinth_btn);

    Column::new()
        .push(title)
        .push(wrap)
        .spacing(10)
        .padding(10)
        .into()
}
