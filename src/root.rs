// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use druid::{
    widget::{Button, Flex, ViewSwitcher},
    Color, Widget, WidgetExt,
};

use crate::{
    about, accounts, creating_instance, install_runtime, instance_name_selection,
    instance_type_selection, instance_version_selection, instances, lib, loading_versions, news,
    runtimes, settings, AppState, View,
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
                    smol::spawn(async move {
                        update_news(event_sink).await;
                    })
                    .detach();
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
        |selector, data, _env| match selector {
            View::Instances => Box::new(instances::build_widget()),
            View::InstanceTypeSelection => Box::new(instance_type_selection::build_widget()),
            View::LoadingVersions => Box::new(loading_versions::build_widget()),
            View::InstanceVersionSelection => Box::new(instance_version_selection::build_widget(
                &data.new_instance_state.shown_minecraft_versions,
            )),
            View::InstanceNameSelection => Box::new(instance_name_selection::build_widget()),
            View::CreatingInstance => Box::new(creating_instance::build_widget()),
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

async fn update_news(event_sink: druid::ExtEventSink) {
    let news = lib::minecraft_news::fetch(None).await.unwrap();

    event_sink.add_idle_callback(move |data: &mut AppState| {
        data.news = news
            .article_grid
            .into_iter()
            .map(|article| (article.default_tile.title, article.article_url))
            .collect();
    });
}
