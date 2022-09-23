// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use druid::{
    im::Vector,
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
                        .with_child(Label::new(|runtime: &String, _env: &_| runtime.to_string()))
                        .with_flex_spacer(1.)
                        .with_child(Button::new("💣 Delete").on_click(
                            |_ctx, runtime: &mut String, _env: &_| {
                                smol::block_on(lib::runtime_manager::remove(runtime)).unwrap();
                            },
                        ))
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
        .with_child(Button::new("Install ⬇️").on_click(new_runtime))
        .padding(10.)
}

fn new_runtime(ctx: &mut druid::EventCtx, data: &mut AppState, _env: &druid::Env) {
    if data.available_runtimes.is_empty() {
        let event_sink = ctx.get_external_handle();
        smol::spawn(update_runtimes(event_sink)).detach();

        data.loading_message = "Loading available runtimes...".to_string();
        data.current_view = View::Loading;
    } else {
        data.current_view = View::InstallRuntime;
    }
}

async fn update_runtimes(event_sink: druid::ExtEventSink) {
    let runtimes = lib::runtime_manager::fetch_available_releases()
        .await
        .unwrap();

    event_sink.add_idle_callback(move |data: &mut AppState| {
        data.available_runtimes = Vector::from(runtimes.available_releases);
        data.current_view = View::InstallRuntime;
    });
}
