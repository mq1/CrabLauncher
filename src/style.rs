use iced::{color, theme, widget::container, Color, Theme};

pub struct Card;

impl container::StyleSheet for Card {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            border_radius: 5.0,
            border_width: 1.0,
            border_color: color!(0x4b5563),
            ..Default::default()
        }
    }
}

pub fn card() -> theme::Container {
    theme::Container::Custom(Box::new(Card))
}
