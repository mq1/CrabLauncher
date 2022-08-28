// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use std::thread;

use druid::{
    im::Vector,
    widget::{Button, CrossAxisAlignment, Flex, Label, List, Scroll},
    Color, Widget, WidgetExt,
};

use crate::{lib, AppState, View};

pub fn build_widget() -> impl Widget<AppState> {
    Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(Label::new("⬇️ Install runtime").with_text_size(32.))
        .with_default_spacer()
        .with_flex_child(
            Scroll::new(
                List::new(|| {
                    Flex::row()
                        .with_child(Label::new(|runtime: &i32, _env: &_| runtime.to_string()))
                        .with_flex_spacer(1.)
                        .with_child(Button::new("Install").on_click(
                            |ctx, runtime: &mut i32, _env: &_| {
                                let event_sink = ctx.get_external_handle();
                                let runtime = runtime.clone();
                                thread::spawn(move || install_runtime(event_sink, &runtime));
                            },
                        ))
                        .padding(5.)
                        .border(Color::GRAY, 1.)
                        .rounded(5.)
                })
                .with_spacing(10.)
                .lens(AppState::available_runtimes),
            )
            .vertical(),
            1.,
        )
        .padding(10.)
}

pub fn update_runtimes(event_sink: druid::ExtEventSink) {
    let runtimes = lib::runtime_manager::fetch_available_releases().unwrap();

    event_sink.add_idle_callback(move |data: &mut AppState| {
        data.available_runtimes = Vector::from(runtimes.available_releases);
    });
}

fn install_runtime(event_sink: druid::ExtEventSink, runtime: &i32) {
    lib::runtime_manager::install(runtime).unwrap();

    event_sink.add_idle_callback(move |data: &mut AppState| {
        data.installed_runtimes = Vector::from(lib::runtime_manager::list().unwrap());
        data.current_view = View::Runtimes;
    });
}
