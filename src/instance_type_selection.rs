// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use druid::{
    im::vector,
    widget::{Button, CrossAxisAlignment, Flex, Label, RadioGroup},
    Color, LensExt, Widget, WidgetExt,
};

use crate::{
    lib::{self, instances::InstanceType},
    AppState, NewInstanceState, View,
};

pub fn build_widget() -> impl Widget<AppState> {
    Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(Label::new("🛠️ Select the instance type").with_text_size(32.))
        .with_flex_spacer(1.)
        .with_child(
            RadioGroup::column(vec![("🍦 Vanilla", InstanceType::Vanilla)])
                .lens(AppState::new_instance_state.then(NewInstanceState::instance_type))
                .padding(5.)
                .border(Color::GRAY, 1.)
                .rounded(5.)
                .expand_width(),
        )
        .with_flex_spacer(1.)
        .with_child(Flex::row().with_flex_spacer(1.).with_child(
            Button::new("Select version 📦 >").on_click(|ctx, data: &mut AppState, _| {
                data.new_instance_state.available_minecraft_versions = vector![];

                let event_sink = ctx.get_external_handle();
                smol::spawn(update_available_versions(event_sink)).detach();
                data.current_view = View::LoadingVersions;
            }),
        ))
        .padding(10.)
}

async fn update_available_versions(event_sink: druid::ExtEventSink) {
    let available_versions = lib::minecraft_version_manifest::fetch_versions()
        .await
        .unwrap();

    event_sink.add_idle_callback(move |data: &mut AppState| {
        data.new_instance_state.available_minecraft_versions = available_versions;
        data.new_instance_state.selected_version =
            Some(data.new_instance_state.available_minecraft_versions[0].clone());
        data.current_view = View::InstanceVersionSelection;
    });
}
