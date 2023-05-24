// SPDX-FileCopyrightText: 2023 Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use iced::{
    color, theme,
    widget::{button, column, svg, text, vertical_space, Button},
    Alignment, Element, Length,
};
use iced_aw::Wrap;

use crate::{util, Message};

fn btn<'a>(label: String, icon: Element<'static, Message>) -> Button<Message> {
    let content = column![
        vertical_space(Length::Fill),
        icon,
        text(label),
        vertical_space(Length::Fill),
    ]
    .align_items(Alignment::Center)
    .spacing(5);

    button(content).height(100).width(100)
}

pub fn view(installers: &Vec<util::lua::Installer>) -> Element<'static, Message> {
    let title = text("New instance").size(30);

    let mut wrap = Wrap::new().spacing(10.);
    for installer in installers {
        let handle = svg::Handle::from_memory(installer.icon_svg.clone());
        let icon = svg(handle)
            .style(theme::Svg::custom_fn(|_theme| svg::Appearance {
                color: Some(color!(0xe2e8f0)),
            }))
            .width(32)
            .height(32)
            .into();

        let button = btn(installer.name.clone(), icon);
        wrap = wrap.push(button);
    }

    column![title, wrap].spacing(10).padding(10).into()
}
