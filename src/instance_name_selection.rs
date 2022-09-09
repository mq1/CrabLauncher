// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use std::thread;

use druid::{
    im::vector,
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
                .with_child(Button::new("< Select version 📦").on_click(
                    |_, data: &mut AppState, _| {
                        data.current_view = View::InstanceVersionSelection;
                    },
                ))
                .with_flex_spacer(1.)
                .with_child(
                    Button::new("Done ✅").on_click(|ctx, data: &mut AppState, _| {
                        let event_sink = ctx.get_external_handle();
                        thread::spawn(move || install_version(event_sink));
                        data.current_view = View::CreatingInstance;
                    }),
                ),
        )
        .padding(10.)
}

fn install_version(event_sink: druid::ExtEventSink) {
    event_sink.add_idle_callback(move |data: &mut AppState| {
        let version = data.new_instance_state.selected_version.clone().unwrap();
        lib::instances::new(&data.new_instance_state.instance_name, &version).unwrap();

        data.new_instance_state.available_minecraft_versions = vector![];

        data.instances = lib::instances::list().unwrap();
        data.current_view = View::Instances;
    });
}
