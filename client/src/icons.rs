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

pub fn rocket() -> Text<'static> {
    icon('\u{eb9b}')
}

pub fn blocks() -> Text<'static> {
    icon('\u{e9b0}')
}
