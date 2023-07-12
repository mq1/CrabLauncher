// SPDX-FileCopyrightText: 2023 Manuel Quarneti <manuq01@pm.me>
// SPDX-License-Identifier: GPL-3.0-only

use iced::{color, theme, widget::svg, Element, Length};

use crate::components::assets;

macro_rules! view {
    ($bytes:expr) => {{
        let handle = svg::Handle::from_memory($bytes);
        svg(handle)
            .style(theme::Svg::custom_fn(|_theme| svg::Appearance {
                color: Some(color!(0xe2e8f0)),
            }))
            .width(Length::Shrink)
            .height(Length::Shrink)
    }};
}

pub fn account_alert<M>() -> Element<'static, M> {
    view!(assets::MDI_ACCOUNT_ALERT_OUTLINE_SVG).into()
}

pub fn account_check<M>() -> Element<'static, M> {
    view!(assets::MDI_ACCOUNT_CHECK_OUTLINE_SVG)
        .width(20)
        .height(20)
        .into()
}

pub fn arrow_left<M>() -> Element<'static, M> {
    view!(assets::MDI_ARROW_LEFT_SVG).into()
}

pub fn cog<M>(dimensions: f32) -> Element<'static, M> {
    view!(assets::MDI_COG_OUTLINE_SVG)
        .width(Length::Fixed(dimensions))
        .height(Length::Fixed(dimensions))
        .into()
}

pub fn content_save<M>() -> Element<'static, M> {
    view!(assets::MDI_CONTENT_SAVE_OUTLINE_SVG).into()
}

pub fn delete<M>() -> Element<'static, M> {
    view!(assets::MDI_DELETE_OUTLINE_SVG)
        .width(20)
        .height(20)
        .into()
}

pub fn github<M>() -> Element<'static, M> {
    view!(assets::MDI_GITHUB_SVG).into()
}

pub fn grid<M>() -> Element<'static, M> {
    view!(assets::MDI_GRID_SVG).into()
}

pub fn info<M>() -> Element<'static, M> {
    view!(assets::MDI_INFORMATION_OUTLINE_SVG).into()
}

pub fn minecraft<M>() -> Element<'static, M> {
    view!(assets::MDI_MINECRAFT_SVG).into()
}

pub fn package<M>() -> Element<'static, M> {
    view!(assets::MDI_PACKAGE_VARIANT_CLOSED_SVG).into()
}

pub fn package_plus<M>() -> Element<'static, M> {
    view!(assets::MDI_PACKAGE_VARIANT_CLOSED_PLUS_SVG).into()
}

pub fn plus<M>() -> Element<'static, M> {
    view!(assets::MDI_PLUS_SVG).into()
}

pub fn rocket<M>() -> Element<'static, M> {
    view!(assets::MDI_ROCKET_LAUNCH_OUTLINE_SVG).into()
}
