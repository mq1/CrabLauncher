// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use druid::{
    widget::{Flex, Label},
    Widget,
};

use crate::AppState;

pub fn build_widget() -> impl Widget<AppState> {
    Flex::column().with_child(Label::new("Update").with_text_size(32.))
}
