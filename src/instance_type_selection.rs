// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use std::thread;

use druid::{
    widget::{Button, CrossAxisAlignment, Flex, Label, MainAxisAlignment},
    Widget, WidgetExt,
};

use crate::{lib, AppState, View};

pub fn build_widget() -> impl Widget<AppState> {
    Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(Label::new("🛠️ Select the instance type").with_text_size(32.))
        .with_flex_spacer(1.)
        .with_child(
            Flex::row()
                .main_axis_alignment(MainAxisAlignment::Center)
                .with_child(Button::new("🍦 Vanilla").on_click(|ctx, _, _| {
                    let event_sink = ctx.get_external_handle();
                    thread::spawn(move || {
                        lib::minecraft_version_manifest::update_available_versions(event_sink)
                    });
                }))
                .with_default_spacer()
                .with_child(
                    Button::<AppState>::new("📦 Modrinth Modpack").on_click(|ctx, _, _| {
                        let event_sink = ctx.get_external_handle();
                        thread::spawn(move || lib::modrinth::update_modpacks(event_sink));
                    }),
                )
                .expand_width(),
        )
        .with_flex_spacer(1.)
        .with_child(
            Flex::row()
                .with_child(
                    Button::<AppState>::new("< Cancel ❌").on_click(|_, data, _| {
                        data.current_view = View::Instances;
                    }),
                )
                .with_flex_spacer(1.),
        )
        .padding(10.)
}
