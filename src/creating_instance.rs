// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use druid::{
    widget::{CrossAxisAlignment, Flex, Label, Spinner},
    UnitPoint, Widget, WidgetExt,
};

use crate::AppState;

pub fn build_widget() -> impl Widget<AppState> {
    let loading = Flex::column()
        .with_child(Label::new("Downloading files..."))
        .with_default_spacer()
        .with_child(Spinner::new())
        .align_horizontal(UnitPoint::CENTER)
        .align_vertical(UnitPoint::CENTER);

    Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(Label::new("🛠️ Creating instance").with_text_size(32.))
        .with_default_spacer()
        .with_flex_child(loading, 1.)
}
