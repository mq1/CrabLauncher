// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use druid::{
    widget::{Flex, Label, ProgressBar},
    UnitPoint, Widget, WidgetExt,
};

use crate::AppState;

pub fn build_widget() -> impl Widget<AppState> {
    Flex::column()
        .with_flex_child(
            Flex::column()
                .with_child(
                    Label::<String>::dynamic(|data, _| data.to_owned())
                        .lens(AppState::loading_message),
                )
                .with_default_spacer()
                .with_child(
                    ProgressBar::new()
                        .lens(AppState::current_progress)
                        .expand_width(),
                )
                .with_default_spacer()
                .with_child(
                    Label::<f64>::dynamic(|data, _| format!("{:.1}%", data * 100.0))
                        .lens(AppState::current_progress),
                )
                .align_horizontal(UnitPoint::CENTER)
                .align_vertical(UnitPoint::CENTER),
            1.,
        )
        .padding(10.)
}
