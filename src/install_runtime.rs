// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use druid::{
    im::Vector,
    widget::{Button, CrossAxisAlignment, Flex, Label, RadioGroup, Scroll},
    Widget, WidgetExt,
};

use crate::{lib, AppState};

pub fn build_widget(available_runtimes: &Vector<i32>) -> impl Widget<AppState> {
    Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(Label::new("⬇️ Install runtime").with_text_size(32.))
        .with_default_spacer()
        .with_child(
            Scroll::new(
                RadioGroup::column(
                    available_runtimes
                        .iter()
                        .map(|r| (r.to_string(), Some(r.to_owned())))
                        .collect::<Vector<_>>(),
                )
                .expand_width()
                .lens(AppState::selected_runtime),
            )
            .vertical(),
        )
        .with_flex_spacer(1.)
        .with_child(Flex::row().with_flex_spacer(1.).with_child(
            Button::<AppState>::new("⬇️ Install").on_click(|ctx, data, _| {
                let runtime = data.selected_runtime.clone().unwrap();
                let event_sink = ctx.get_external_handle();
                tokio::spawn(lib::runtime_manager::install(runtime, event_sink));
            }),
        ))
        .padding(10.)
}
