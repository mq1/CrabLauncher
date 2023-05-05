// SPDX-FileCopyrightText: 2023 Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use iced::{
    color, theme,
    widget::{button, container},
    Background, Theme,
};
pub struct Card {
    theme: theme::Container,
}

impl Card {
    pub fn new(theme: theme::Container) -> Self {
        Self { theme }
    }
}

impl container::StyleSheet for Card {
    type Style = Theme;

    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        let mut appearance = style.appearance(&self.theme);
        appearance.border_radius = 5.0;
        appearance.border_width = 1.0;
        appearance.border_color = color!(0x3f3f46);
        appearance.background = Some(Background::Color(color!(0x27272a)));

        appearance
    }
}

pub fn card() -> theme::Container {
    theme::Container::Custom(Box::new(Card::new(theme::Container::default())))
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
        appearance.border_radius = 200.0;

        appearance
    }

    fn disabled(&self, style: &Self::Style) -> button::Appearance {
        let mut appearance = style.disabled(&self.theme);
        appearance.border_radius = 200.0;

        appearance
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        let mut appearance = style.hovered(&self.theme);
        appearance.border_radius = 200.0;

        appearance
    }

    fn pressed(&self, style: &Self::Style) -> button::Appearance {
        let mut appearance = style.pressed(&self.theme);
        appearance.border_radius = 200.0;

        appearance
    }
}

pub fn circle_button() -> theme::Button {
    theme::Button::Custom(Box::new(CircleButtonStyle::new(theme::Button::Primary)))
}
