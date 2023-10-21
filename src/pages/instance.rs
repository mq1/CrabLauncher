// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuelquarneti@protonmail.com>
// SPDX-License-Identifier: GPL-3.0-only

use iced::{Alignment, Color, Element, Length, theme, widget::{button, Column, horizontal_space, Row, text, vertical_space}};

use crate::{assets, components::icons, Message, style, util::instances::Instance};

pub fn view(instance: &Instance) -> Element<Message> {
    let image = icons::view_png(assets::GRASS_PNG, 128);
    let minecraft_version = text(format!("Minecraft {}", instance.info.minecraft))
        .style(theme::Text::Color(Color::from_rgb8(175, 175, 175)));

    let name = text(instance.name.clone()).size(50);

    let play_button = button(
        Row::new()
            .push(text("Launch").size(30))
            .push(icons::view(icons::ROCKET_LAUNCH_OUTLINE))
            .padding(10)
            .spacing(10)
            .align_items(Alignment::Center)
    )
        .on_press(Message::LaunchInstance(instance.to_owned()))
        .style(style::circle_button(theme::Button::Primary));

    let delete_button = button(
        Row::new()
            .push(text(" Delete instance "))
            .push(icons::view(icons::DELETE_OUTLINE))
            .padding(5)
            .align_items(Alignment::Center)
    )
        .style(style::circle_button(theme::Button::Destructive))
        .on_press(Message::DeleteInstance(instance.to_owned()));

    let edit_button = button(
        Row::new().push(text(" Edit instance ")).push(icons::view(icons::COG_OUTLINE))
            .padding(5)
            .align_items(Alignment::Center),
    )
        .style(style::circle_button(theme::Button::Primary));

    let footer = Row::new()
        .push(horizontal_space(Length::Fill))
        .push(delete_button)
        .push(edit_button)
        .spacing(10)
        .padding(10);

    Column::new()
        .push(vertical_space(Length::Fill))
        .push(Row::new()
            .push(Column::new()
                .push(image)
                .push(minecraft_version)
                .spacing(10)
                .align_items(Alignment::Center)
            )
            .push(Column::new()
                .push(name)
                .push(play_button)
                .spacing(20)
                .align_items(Alignment::Center)
            )
            .spacing(50)
            .align_items(Alignment::Center)
        )
        .push(vertical_space(Length::Fill))
        .push(footer)
        .align_items(Alignment::Center)
        .width(Length::Fill)

        .spacing(50)
        .into()
}
