// SPDX-FileCopyrightText: 2023 Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use iced::{
    widget::{button, column, text, Button},
    Alignment, Element,
};
use iced_aw::Wrap;

use crate::{components::icons, Message, View};

fn btn<'a>(label: &'a str, icon: Element<'static, Message>, view: View) -> Button<'a, Message> {
    let content = column![icon, text(label)]
        .align_items(Alignment::Center)
        .padding(5);

    button(content).on_press(Message::ChangeView(view))
}

pub fn view() -> Element<'static, Message> {
    let title = text("New instance").size(30);

    let mut wrap = Wrap::new().spacing(10.);
    {
        let vanilla_button = btn("Vanilla", icons::minecraft(), View::NewVanillaInstance);
        let modrinth_button = btn("Modrinth", icons::modrinth(), View::NewModrinthInstance);

        wrap = wrap.push(vanilla_button);
        wrap = wrap.push(modrinth_button);
    }

    column![title, wrap].spacing(10).padding(10).into()
}
