// SPDX-FileCopyrightText: 2023 Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use iced::{
    color, theme,
    widget::{button, column, svg, text, vertical_space, Button},
    Alignment, Element, Length,
};
use iced_aw::Wrap;

use crate::{util, Message};

fn btn<'a>(
    installer: &util::lua::InstallerInfo,
    icon: Element<'static, Message>,
) -> Button<'a, Message> {
    let content = column![
        vertical_space(Length::Fill),
        icon,
        text(&installer.name),
        vertical_space(Length::Fill),
    ]
    .align_items(Alignment::Center)
    .spacing(5);

    button(content)
        .height(100)
        .width(100)
        .on_press(Message::SelectInstaller(installer.to_owned()))
}

pub fn view(installers: &util::lua::InstallersIndex) -> Element<'static, Message> {
    let title = text("New instance").size(30);

    let content: Element<Message> = if installers.is_empty() {
        text("Fetching installers...").into()
    } else {
        let mut wrap = Wrap::new().spacing(10.);
        for installer in installers {
            let icon_bytes = installer.icon_svg.as_bytes().to_vec();
            let handle = svg::Handle::from_memory(icon_bytes);
            let icon = svg(handle)
                .style(theme::Svg::custom_fn(|_theme| svg::Appearance {
                    color: Some(color!(0xe2e8f0)),
                }))
                .width(32)
                .height(32)
                .into();

            let button = btn(installer, icon);
            wrap = wrap.push(button);
        }

        wrap.into()
    };

    column![title, content].spacing(10).padding(10).into()
}
