// SPDX-FileCopyrightText: 2023 Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use iced::{
    color, theme,
    widget::{button, column, svg, text, vertical_space, Button},
    Alignment, Element, Length,
};
use iced_aw::Wrap;

use crate::{components::assets::MDI_MINECRAFT_SVG, Message, View};

fn btn(name: &str, installer_view: View, icon_svg: &[u8]) -> Button<'static, Message> {
    let bytes = icon_svg.to_vec();
    let handle = svg::Handle::from_memory(bytes);
    let icon = svg(handle)
        .style(theme::Svg::custom_fn(|_theme| svg::Appearance {
            color: Some(color!(0xe2e8f0)),
        }))
        .width(32)
        .height(32);

    let content = column![
        vertical_space(Length::Fill),
        icon,
        text(name),
        vertical_space(Length::Fill),
    ]
    .align_items(Alignment::Center)
    .spacing(5);

    button(content)
        .height(100)
        .width(100)
        .on_press(Message::ChangeView(installer_view))
}

pub fn view() -> Element<'static, Message> {
    let title = text("New instance").size(30);

    let mut wrap = Wrap::new().spacing(10.);

    // Vanilla
    let vanilla_btn = btn("Vanilla", View::VanillaInstaller, MDI_MINECRAFT_SVG);
    wrap = wrap.push(vanilla_btn);

    column![title, wrap].spacing(10).padding(10).into()
}
