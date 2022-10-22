// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use druid::{
    widget::{Flex, Label, Spinner},
    UnitPoint, Widget, WidgetExt,
};

use crate::AppState;

pub fn build_widget() -> impl Widget<AppState> {
    Flex::column()
        .with_flex_child(
            Flex::column()
                .with_child(Label::<AppState>::dynamic(|data, _| {
                    data.current_message.to_owned()
                }))
                .with_default_spacer()
                .with_child(Spinner::new())
                .align_horizontal(UnitPoint::CENTER)
                .align_vertical(UnitPoint::CENTER),
            1.,
        )
        .padding(10.)
}
