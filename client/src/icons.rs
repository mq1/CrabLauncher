// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use iced::{
    widget::{text, Text},
    Font,
};

// Fonts
const ICONS: Font = Font::External {
    name: "Icons",
    bytes: include_bytes!("../../assets/MaterialIcons-Regular.ttf"),
};

fn icon(unicode: char) -> Text<'static> {
    text(unicode.to_string()).font(ICONS)
}

pub fn delete() -> Text<'static> {
    icon('\u{e872}')
}

pub fn rocket_launch() -> Text<'static> {
    icon('\u{eb9b}')
}

pub fn grid_view() -> Text<'static> {
    icon('\u{e9b0}')
}

pub fn manage_accounts() -> Text<'static> {
    icon('\u{f02e}')
}

pub fn newspaper() -> Text<'static> {
    icon('\u{eb81}')
}

pub fn open_in_new() -> Text<'static> {
    icon('\u{e89e}')
}
