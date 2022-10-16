// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use druid::{
    widget::{Button, CrossAxisAlignment, Flex, Label, TextBox},
    Color, LensExt, Widget, WidgetExt,
};

use crate::{lib, AppState, NewInstanceState, View};

pub fn build_widget() -> impl Widget<AppState> {
    Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(Label::new("✍️ Type a name for your new instance").with_text_size(32.))
        .with_flex_spacer(1.)
        .with_child(
            TextBox::new()
                .with_placeholder("My new Instance")
                .lens(AppState::new_instance_state.then(NewInstanceState::instance_name))
                .padding(5.)
                .border(Color::GRAY, 1.)
                .rounded(5.)
                .expand_width(),
        )
        .with_flex_spacer(1.)
        .with_child(
            Flex::row()
                .with_child(Button::<AppState>::new("< Select version 📦").on_click(
                    |_, data, _| {
                        data.current_view = View::InstanceVersionSelection;
                    },
                ))
                .with_flex_spacer(1.)
                .with_child(Button::<AppState>::new("Done ✅").on_click(|ctx, data, _| {
                    let name = data.new_instance_state.instance_name.clone();
                    let version = data.new_instance_state.selected_version.clone().unwrap();
                    let event_sink = ctx.get_external_handle();

                    tokio::spawn(lib::instances::new(name, version, event_sink));
                })),
        )
        .padding(10.)
}
