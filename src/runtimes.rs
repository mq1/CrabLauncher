// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use druid::{
    widget::{Button, CrossAxisAlignment, Flex, Label, List, Scroll},
    Color, Widget, WidgetExt,
};

use crate::{lib, AppState, View};

pub fn build_widget() -> impl Widget<AppState> {
    Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(Label::new("🚀 Runtimes").with_text_size(32.))
        .with_default_spacer()
        .with_flex_child(
            Scroll::new(
                List::new(|| {
                    Flex::row()
                        .with_child(Label::new("☕️"))
                        .with_default_spacer()
                        .with_child(Label::<String>::dynamic(|runtime, _| runtime.to_string()))
                        .with_flex_spacer(1.)
                        .with_child(
                            Button::<String>::new("💣 Delete").on_click(|_, runtime, _| {
                                smol::block_on(lib::runtime_manager::remove(runtime)).unwrap();
                            }),
                        )
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
            Button::<AppState>::new("Install ⬇️").on_click(|ctx, data, _| {
                if data.available_runtimes.is_empty() {
                    data.loading_message = "Loading available runtimes...".to_string();
                    data.current_view = View::Loading;

                    let event_sink = ctx.get_external_handle();
                    smol::spawn(update_runtimes(event_sink)).detach();
                } else {
                    data.current_view = View::InstallRuntime;
                }
            }),
        )
        .padding(10.)
}

async fn update_runtimes(event_sink: druid::ExtEventSink) {
    let runtimes = lib::runtime_manager::fetch_available_releases()
        .await
        .unwrap();

    event_sink.add_idle_callback(move |data: &mut AppState| {
        data.available_runtimes = runtimes.available_releases;
        data.current_view = View::InstallRuntime;
    });
}
