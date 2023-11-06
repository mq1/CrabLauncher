// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuelquarneti@protonmail.com>
// SPDX-License-Identifier: GPL-3.0-only

use iced::widget::{button, container};
use iced::{theme, Background, Color, Theme};

// -------------------------------------------------------------------------------------------------

pub struct NavbarContainerStyle;

impl container::StyleSheet for NavbarContainerStyle {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(Background::Color(Color::from_rgb8(24, 24, 27))),
            text_color: Some(Color::WHITE),
            ..container::Appearance::default()
        }
    }
}

pub fn navbar_container() -> theme::Container {
    theme::Container::Custom(Box::new(NavbarContainerStyle))
}

// -------------------------------------------------------------------------------------------------

pub enum NavbarButtonStyle {
    Normal,
    Selected,
}

impl button::StyleSheet for NavbarButtonStyle {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        match self {
            NavbarButtonStyle::Normal => button::Appearance {
                background: Some(Background::Color(Color::TRANSPARENT)),
                text_color: Color::WHITE,
                ..Default::default()
            },
            NavbarButtonStyle::Selected => button::Appearance {
                background: Some(Background::Color(Color::from_rgb8(249, 115, 22))),
                text_color: Color::WHITE,
                ..Default::default()
            },
        }
    }

    fn hovered(&self, _style: &Self::Style) -> button::Appearance {
        self.active(_style)
    }

    fn pressed(&self, _style: &Self::Style) -> button::Appearance {
        self.active(_style)
    }
}

pub fn navbar_button(style: NavbarButtonStyle) -> theme::Button {
    theme::Button::Custom(Box::new(style))
}

// -------------------------------------------------------------------------------------------------

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
        appearance.border_radius = 200.0.into();

        appearance
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        let mut appearance = style.hovered(&self.theme);
        appearance.border_radius = 200.0.into();

        appearance
    }

    fn pressed(&self, style: &Self::Style) -> button::Appearance {
        let mut appearance = style.pressed(&self.theme);
        appearance.border_radius = 200.0.into();

        appearance
    }

    fn disabled(&self, style: &Self::Style) -> button::Appearance {
        let mut appearance = style.disabled(&self.theme);
        appearance.border_radius = 200.0.into();

        appearance
    }
}

pub fn circle_button(theme: theme::Button) -> theme::Button {
    theme::Button::Custom(Box::new(CircleButtonStyle::new(theme)))
}

// -------------------------------------------------------------------------------------------------
