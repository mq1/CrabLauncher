// SPDX-FileCopyrightText: 2023 Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use std::{fs, path::PathBuf};

use iced::{
    color, theme,
    widget::{button, column, svg, text, Button},
    Alignment, Element, Length,
};
use iced_aw::Wrap;

use crate::Message;

fn btn<'a>(label: String, icon: Element<'static, Message>) -> Button<Message> {
    let content = column![icon, text(label)]
        .align_items(Alignment::Center)
        .padding(5);

    button(content)
}

pub fn view(installers: &Vec<PathBuf>, lua: &mlua::Lua) -> Element<'static, Message> {
    let title = text("New instance").size(30);

    let mut wrap = Wrap::new().spacing(10.);
    for installer in installers {
        let str = fs::read_to_string(installer).unwrap();
        lua.load(&str).exec().unwrap();

        let name = lua.globals().get::<_, String>("Name").unwrap();
        let icon_svg = lua.globals().get::<_, String>("IconSVG").unwrap();
        let icon_bytes = icon_svg.as_bytes().to_vec();

        let handle = svg::Handle::from_memory(icon_bytes);
        let icon = svg(handle)
            .style(theme::Svg::custom_fn(|_theme| svg::Appearance {
                color: Some(color!(0xe2e8f0)),
            }))
            .width(Length::Shrink)
            .height(Length::Shrink)
            .into();

        let button = btn(name, icon);
        wrap = wrap.push(button);
    }

    column![title, wrap].spacing(10).padding(10).into()
}
