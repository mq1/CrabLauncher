// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use std::thread;

use druid::{
    widget::{Button, Flex, ViewSwitcher},
    Color, Widget, WidgetExt,
};

use crate::{
    about, accounts, install_runtime, instances, news, runtimes, settings, AppState, View, create_instance,
};

pub fn build_widget() -> impl Widget<AppState> {
    let switcher_column = Flex::column()
        .with_child(
            Button::new("Instances").on_click(|_ctx, data: &mut AppState, _env| {
                data.current_view = View::Instances;
            }),
        )
        .with_default_spacer()
        .with_child(
            Button::new("Accounts").on_click(|_ctx, data: &mut AppState, _env| {
                data.current_view = View::Accounts;
            }),
        )
        .with_default_spacer()
        .with_child(
            Button::new("Runtimes").on_click(|_ctx, data: &mut AppState, _env| {
                data.current_view = View::Runtimes;
            }),
        )
        .with_default_spacer()
        .with_child(
            Button::new("News").on_click(|ctx, data: &mut AppState, _env| {
                if data.news.is_empty() {
                    let event_sink = ctx.get_external_handle();
                    thread::spawn(move || news::update_news(event_sink));
                }
                data.current_view = View::News;
            }),
        )
        .with_flex_spacer(1.)
        .with_child(
            Button::new("Settings").on_click(|_ctx, data: &mut AppState, _env| {
                data.current_view = View::Settings;
            }),
        )
        .with_default_spacer()
        .with_child(
            Button::new("About").on_click(|_ctx, data: &mut AppState, _env| {
                data.current_view = View::About;
            }),
        )
        .padding(10.)
        .background(Color::from_hex_str("#404040").unwrap());

    let view_switcher = ViewSwitcher::new(
        |data: &AppState, _env| data.current_view,
        |selector, _data, _env| match selector {
            View::Instances => Box::new(instances::build_widget()),
            View::CreateInstance => Box::new(create_instance::build_widget()),
            View::Accounts => Box::new(accounts::build_widget()),
            View::Runtimes => Box::new(runtimes::build_widget()),
            View::InstallRuntime => Box::new(install_runtime::build_widget()),
            View::News => Box::new(news::build_widget()),
            View::Settings => Box::new(settings::build_widget()),
            View::About => Box::new(about::build_widget()),
        },
    );

    Flex::row()
        .with_child(switcher_column)
        .with_flex_child(view_switcher, 1.0)
        .expand_height()
}
