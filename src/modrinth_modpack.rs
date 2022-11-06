// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use druid::{
    widget::{CrossAxisAlignment, Flex, Label},
    Widget, WidgetExt,
};

use crate::{lib::modrinth::Hit, AppState};

pub fn build_widget() -> impl Widget<AppState> {
    Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(
            Label::<Option<Hit>>::dynamic(|data, _| data.as_ref().unwrap().title.to_owned())
                .with_text_size(32.),
        )
        .with_flex_spacer(1.)
        .padding(10.)
        .lens(AppState::selected_modrinth_hit)
}
