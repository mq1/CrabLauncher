// SPDX-FileCopyrightText: 2023 Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use iced::{
    color, theme,
    widget::{button, column, svg, text, vertical_space, Button},
    Alignment, Element, Length,
};
use iced_aw::Wrap;

use crate::Message;

fn btn<'a>(name: &str, index: usize, icon: Element<'static, Message>) -> Button<'a, Message> {
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
        .on_press(Message::SelectInstaller(index))
}

pub fn view(installers: &Vec<mlua::Lua>) -> Element<'static, Message> {
    let title = text("New instance").size(30);

    let mut wrap = Wrap::new().spacing(10.);
    for (index, installer) in installers.iter().enumerate() {
        let icon_bytes = installer
            .globals()
            .get::<_, String>("IconSVG")
            .unwrap()
            .as_bytes()
            .to_vec();
        let handle = svg::Handle::from_memory(icon_bytes);
        let icon = svg(handle)
            .style(theme::Svg::custom_fn(|_theme| svg::Appearance {
                color: Some(color!(0xe2e8f0)),
            }))
            .width(32)
            .height(32)
            .into();

        let name = installer.globals().get::<_, String>("Name").unwrap();
        let button = btn(&name, index, icon);
        wrap = wrap.push(button);
    }

    column![title, wrap].spacing(10).padding(10).into()
}
