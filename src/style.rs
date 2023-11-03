// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuelquarneti@protonmail.com>
// SPDX-License-Identifier: GPL-3.0-only

use iced::widget::button::Appearance;
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

    fn active(&self, _style: &Self::Style) -> Appearance {
        match self {
            NavbarButtonStyle::Normal => Appearance {
                background: Some(Background::Color(Color::TRANSPARENT)),
                text_color: Color::WHITE,
                ..Appearance::default()
            },
            NavbarButtonStyle::Selected => Appearance {
                background: Some(Background::Color(Color::from_rgb8(63, 63, 70))),
                text_color: Color::WHITE,
                ..Appearance::default()
            },
        }
    }

    fn hovered(&self, _style: &Self::Style) -> Appearance {
        self.active(_style)
    }

    fn pressed(&self, _style: &Self::Style) -> Appearance {
        self.active(_style)
    }
}

pub fn navbar_button(style: NavbarButtonStyle) -> theme::Button {
    theme::Button::Custom(Box::new(style))
}

// -------------------------------------------------------------------------------------------------
