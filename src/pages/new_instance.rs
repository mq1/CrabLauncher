// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuq01@pm.me>
// SPDX-License-Identifier: GPL-3.0-only

use iced::{
    Alignment,
    Element, Length, widget::{button, Button, Column, text, vertical_space},
};
use iced_aw::Wrap;

use crate::{components::icons, Message, pages::Page};

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
    let vanilla_btn = installer_button(
        "Vanilla",
        Page::VanillaInstaller,
        icons::view_png(icons::GRASS_PNG, 64),
    );
    wrap = wrap.push(vanilla_btn);

    // Modrinth
    let modrinth_btn = installer_button(
        "Modrinth",
        Page::ModrinthModpacks,
        icons::view_custom(icons::MODRINTH, 64),
    );
    wrap = wrap.push(modrinth_btn);

    Column::new()
        .push(title)
        .push(wrap)
        .spacing(10)
        .padding(10)
        .into()
}
