// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use druid::{
    im::Vector,
    lens,
    widget::{Button, CrossAxisAlignment, Flex, Label, List, Scroll},
    Color, LensExt, Widget, WidgetExt,
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
                        .with_child(Label::<(_, String)>::dynamic(|(_, runtime), _| {
                            runtime.to_string()
                        }))
                        .with_flex_spacer(1.)
                        .with_child(
                            Button::<(Vector<String>, String)>::new("💣 Delete").on_click(
                                |_, (runtimes, runtime), _| {
                                    tokio::spawn(lib::runtime_manager::remove(runtime.clone()));
                                    runtimes.retain(|r| r != runtime);
                                },
                            ),
                        )
                        .padding(5.)
                        .border(Color::GRAY, 1.)
                        .rounded(5.)
                })
                .with_spacing(10.)
                .lens(lens::Identity.map(
                    |data: &AppState| {
                        (
                            data.installed_runtimes.clone(),
                            data.installed_runtimes.clone(),
                        )
                    },
                    |data: &mut AppState, (installed_runtimes, _)| {
                        data.installed_runtimes = installed_runtimes;
                    },
                )),
            )
            .vertical(),
            1.,
        )
        .with_default_spacer()
        .with_child(
            Button::<AppState>::new("Install ⬇️").on_click(|ctx, data, _| {
                if data.available_runtimes.is_empty() {
                    let event_sink = ctx.get_external_handle();
                    tokio::spawn(lib::runtime_manager::update_runtimes(event_sink));
                } else {
                    data.current_view = View::InstallRuntime;
                }
            }),
        )
        .padding(10.)
}
