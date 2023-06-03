// SPDX-FileCopyrightText: 2023 Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use iced::{
    color, theme,
    widget::{button, column, svg, text, vertical_space, Button},
    Alignment, Element, Length,
};
use iced_aw::Wrap;

use crate::{
    util::{self, lua::INSTALLERS},
    Message,
};

fn btn(installer: &str) -> Button<'static, Message> {
    let info = util::lua::get_installer_info(installer).unwrap();

    let bytes = info.icon_svg.as_bytes().to_vec();
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
        text(info.name),
        vertical_space(Length::Fill),
    ]
    .align_items(Alignment::Center)
    .spacing(5);

    button(content)
        .height(100)
        .width(100)
        .on_press(Message::SelectInstaller(installer.to_owned()))
}

pub fn view() -> Element<'static, Message> {
    let title = text("New instance").size(30);

    let mut wrap = Wrap::new().spacing(10.);
    for installer in INSTALLERS.keys() {
        let button = btn(installer);
        wrap = wrap.push(button);
    }

    column![title, wrap].spacing(10).padding(10).into()
}
