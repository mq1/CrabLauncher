// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use std::thread;

use druid::{
    im::vector,
    widget::{Button, CrossAxisAlignment, Flex, Label, RadioGroup},
    Color, Widget, WidgetExt,
};

use crate::{
    lib::{self, instances::InstanceType},
    AppState, View,
};

pub fn build_widget() -> impl Widget<AppState> {
    Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(Label::new("🛠️ Select the instance type").with_text_size(32.))
        .with_flex_spacer(1.)
        .with_child(
            RadioGroup::column(vec![
                ("🍦 Vanilla", InstanceType::Vanilla),
                ("🧵 Fabric", InstanceType::Fabric),
                ("🔥 Forge", InstanceType::Forge),
            ])
            .lens(AppState::new_instance_type)
            .padding(5.)
            .border(Color::GRAY, 1.)
            .rounded(5.)
            .expand_width(),
        )
        .with_flex_spacer(1.)
        .with_child(Flex::row().with_flex_spacer(1.).with_child(
            Button::new("Select version 📦 >").on_click(|ctx, data: &mut AppState, _| {
                let event_sink = ctx.get_external_handle();
                thread::spawn(move || update_available_versions(event_sink));
                data.current_view = View::InstanceVersionSelection;
            }),
        ))
        .padding(10.)
}

fn update_available_versions(event_sink: druid::ExtEventSink) {
    event_sink.add_idle_callback(move |data: &mut AppState| {
        data.available_minecraft_versions = vector![];
    });

    let available_versions = lib::minecraft_version_manifest::fetch_versions().unwrap();
    let versions = available_versions
        .iter()
        .map(|v| (v.id.clone(), false))
        .collect();

    event_sink.add_idle_callback(move |data: &mut AppState| {
        data.available_minecraft_versions = available_versions;
        data.version_selection = versions;
    });
}