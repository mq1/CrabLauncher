// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use std::thread;

use druid::{
    widget::{Button, CrossAxisAlignment, Flex, Label, List, Scroll},
    Color, Widget, WidgetExt,
};

use crate::{install_runtime, AppState, View};

pub fn build_widget() -> impl Widget<AppState> {
    Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(Label::new("🚀 Runtimes").with_text_size(32.))
        .with_default_spacer()
        .with_flex_child(
            Scroll::new(
                List::new(|| {
                    Flex::row()
                        .with_child(Label::new(|runtime: &String, _env: &_| runtime.to_string()))
                        .with_flex_spacer(1.)
                        .padding(5.)
                        .border(Color::GRAY, 1.)
                        .rounded(5.)
                })
                .with_spacing(10.)
                .lens(AppState::installed_runtimes),
            )
            .vertical(),
            1.,
        )
        .with_default_spacer()
        .with_child(
            Button::new("Install ⬇️").on_click(|ctx, data: &mut AppState, _env| {
                if data.available_runtimes.is_empty() {
                    let event_sink = ctx.get_external_handle();
                    thread::spawn(move || install_runtime::update_runtimes(event_sink));
                }

                data.current_view = View::InstallRuntime;
            }),
        )
        .padding(10.)
}
