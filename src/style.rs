// SPDX-FileCopyrightText: 2023 Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use iced::{
    theme::{self, Palette},
    widget::{button, container},
    Background, Theme, color,
};

// Catppuccin Mocha Sapphire
pub fn my_theme() -> Theme {
    Theme::custom(Palette {
        background: color!(0x1e1e2e),
        text: color!(0xcdd6f4),
        primary: color!(0x74c7ec),
        success: color!(0xa6e3a1),
        danger: color!(0xf38ba8),
    })
}

pub struct Card;

impl container::StyleSheet for Card {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            border_radius: 5.0,
            border_width: 1.0,
            border_color: color!(0x3f3f46),
            background: Some(Background::Color(color!(0x313244))),
            ..Default::default()
        }
    }
}

pub fn card() -> theme::Container {
    theme::Container::Custom(Box::new(Card))
}

pub struct CircleButtonStyle {
    theme: theme::Button,
}

impl CircleButtonStyle {
    pub fn new(theme: theme::Button) -> Self {
        Self { theme }
    }
}

impl button::StyleSheet for CircleButtonStyle {
    type Style = Theme;

    fn active(&self, style: &Self::Style) -> button::Appearance {
        let mut appearance = style.active(&self.theme);
        appearance.text_color = color!(0x1e1e2e);
        appearance.border_radius = 200.0;

        appearance
    }

    fn disabled(&self, style: &Self::Style) -> button::Appearance {
        let mut appearance = style.disabled(&self.theme);
        appearance.text_color = color!(0x313244);
        appearance.border_radius = 200.0;

        appearance
    }
}

pub fn circle_button() -> theme::Button {
    theme::Button::Custom(Box::new(CircleButtonStyle::new(theme::Button::Primary)))
}
