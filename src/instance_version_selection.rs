// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use druid::{
    widget::{Button, CrossAxisAlignment, Flex, Label, List, Scroll},
    Color, LensExt, Widget, WidgetExt,
};

use crate::{lib::minecraft_version_manifest::Version, AppState, NewInstanceState, View};

pub fn build_widget() -> impl Widget<AppState> {
    Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(Label::new("📦 Select the version").with_text_size(32.))
        .with_default_spacer()
        .with_flex_child(
            Scroll::new(
                List::new(|| {
                    Flex::row()
                        .with_child(Label::<Version>::dynamic(|data, _| data.id.to_owned()))
                        .with_flex_spacer(1.)
                        .with_child(Button::<Version>::new("Select").on_click(|ctx, data, _| {
                            let version = data.clone();
                            let event_sink = ctx.get_external_handle();
                            event_sink.add_idle_callback(move |data: &mut AppState| {
                                data.new_instance_state.selected_version = Some(version);
                                data.current_view = View::InstanceNameSelection;
                            });
                        }))
                        .padding(5.)
                        .border(Color::GRAY, 1.)
                        .rounded(5.)
                })
                .with_spacing(10.)
                .lens(
                    AppState::new_instance_state
                        .then(NewInstanceState::available_minecraft_versions),
                ),
            )
            .vertical(),
            1.,
        )
        .with_default_spacer()
        .with_child(
            Flex::row()
                .with_child(
                    Button::<AppState>::new("< Select type 🛠️").on_click(|_, data, _| {
                        data.current_view = View::InstanceTypeSelection;
                    }),
                )
                .with_flex_spacer(1.),
        )
        .padding(10.)
}
