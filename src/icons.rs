// SPDX-FileCopyrightText: 2023 Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use iced::{color, theme, widget::svg, Element};

use crate::{assets, Message};

macro_rules! view {
    ($bytes:expr) => {{
        let handle = svg::Handle::from_memory($bytes);
        svg(handle)
            .width(20)
            .height(20)
            .style(theme::Svg::custom_fn(|_theme| svg::Appearance {
                color: Some(color!(0xe2e8f0)),
            }))
            .into()
    }};
}

pub fn cog() -> Element<'static, Message> {
    view!(assets::COG_SVG)
}
