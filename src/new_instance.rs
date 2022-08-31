// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use std::thread;

use druid::{
    widget::{Button, CrossAxisAlignment, Either, Flex, Label, List, Scroll, Spinner, TextBox},
    Color, UnitPoint, Widget, WidgetExt,
};

use crate::{
    lib::{self, minecraft_version_manifest::Version},
    AppState, View,
};

pub fn build_widget() -> impl Widget<AppState> {
    let installing = Flex::column()
        .with_child(Label::new("Installing version..."))
        .with_default_spacer()
        .with_child(Spinner::new())
        .align_horizontal(UnitPoint::CENTER)
        .align_vertical(UnitPoint::CENTER);

    let loading_versions = Flex::column()
        .with_child(Label::new("Fetching available versions..."))
        .with_default_spacer()
        .with_child(Spinner::new())
        .align_horizontal(UnitPoint::CENTER)
        .align_vertical(UnitPoint::CENTER);

    let instance_name_selector = TextBox::new()
        .with_placeholder("Instance name")
        .lens(AppState::new_instance_name)
        .padding(5.)
        .border(Color::GRAY, 1.)
        .rounded(5.)
        .expand_width();

    let version_selector = Scroll::new(
        List::new(|| {
            Flex::row()
                .with_child(Label::new(|version: &Version, _env: &_| {
                    version.id.to_owned()
                }))
                .with_flex_spacer(1.)
                .with_child(Button::new("Install").on_click(
                    |ctx, version: &mut Version, _env: &_| {
                        let event_sink = ctx.get_external_handle();
                        select_version(event_sink.clone(), version.to_owned());
                        thread::spawn(move || install_version(event_sink));
                    },
                ))
                .padding(5.)
                .border(Color::GRAY, 1.)
                .rounded(5.)
        })
        .with_spacing(10.)
        .lens(AppState::available_minecraft_versions),
    )
    .vertical();

    let parameters_selection = Flex::column()
        .with_child(instance_name_selector)
        .with_default_spacer()
        .with_flex_child(version_selector, 1.);

    let either_versions = Either::new(
        |data, _env| data.available_minecraft_versions.is_empty(),
        loading_versions,
        parameters_selection,
    );

    let either = Either::new(
        |data, _env| data.installing_version,
        installing,
        either_versions,
    );

    Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(Label::new("✨ Create a new Instance").with_text_size(32.))
        .with_default_spacer()
        .with_flex_child(either, 1.)
        .padding(10.)
}

pub fn update_available_versions(event_sink: druid::ExtEventSink) {
    let versions = lib::minecraft_version_manifest::fetch_versions().unwrap();

    event_sink.add_idle_callback(move |data: &mut AppState| {
        data.available_minecraft_versions = versions;
    });
}

fn select_version(event_sink: druid::ExtEventSink, version: Version) {
    event_sink.add_idle_callback(move |data: &mut AppState| {
        data.selected_version = Some(version);
    });
}

fn install_version(event_sink: druid::ExtEventSink) {
    event_sink.add_idle_callback(move |data: &mut AppState| {
        data.installing_version = true;
        lib::instances::new(
            &data.new_instance_name,
            data.selected_version.as_ref().unwrap(),
        )
        .unwrap();
        data.installing_version = false;
        data.instances = lib::instances::list().unwrap();
        data.current_view = View::Instances;
    });
}
