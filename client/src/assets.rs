// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use iced::Font;

pub const LOGO_PNG: &[u8] = include_bytes!("../../assets/ice-launcher.png");

pub const ICONS: Font = Font::External {
    name: "Icons",
    bytes: include_bytes!("../../assets/MaterialIcons-Regular.ttf"),
};
