// SPDX-FileCopyrightText: 2023 Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use iced::{color, theme, widget::container, Background, Theme};

pub struct Card;

impl container::StyleSheet for Card {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            border_radius: 5.0,
            border_width: 1.0,
            border_color: color!(0x3f3f46),
            background: Some(Background::Color(color!(0x27272a))),
            ..Default::default()
        }
    }
}

pub fn card() -> theme::Container {
    theme::Container::Custom(Box::new(Card))
}
