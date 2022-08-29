// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use druid::{Widget, widget::{Flex, CrossAxisAlignment, Label}};

use crate::AppState;

pub fn build_widget() -> impl Widget<AppState> {
    Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(Label::new("✨ Create a new Instance").with_text_size(32.))
}
