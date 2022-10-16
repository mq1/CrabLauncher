// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use druid::{
    widget::{Button, Flex, ViewSwitcher},
    Color, Widget, WidgetExt,
};

use crate::{
    about, accounts, install_runtime, instance_name_selection, instance_type_selection,
    instance_version_selection, instances, lib, loading, news, progress, runtimes, settings,
    AppState, View,
};

pub fn build_widget() -> impl Widget<AppState> {
    let switcher_column = Flex::column()
        .with_child(Button::<AppState>::new("Instances").on_click(|_, data, _| {
            data.current_view = View::Instances;
        }))
        .with_default_spacer()
        .with_child(Button::<AppState>::new("Runtimes").on_click(|_, data, _| {
            data.current_view = View::Runtimes;
        }))
        .with_default_spacer()
        .with_child(Button::<AppState>::new("News").on_click(|ctx, data, _| {
            if data.news.article_count == 0 {
                let event_sink = ctx.get_external_handle();
                tokio::spawn(lib::minecraft_news::update_news(event_sink));
            } else {
                data.current_view = View::News;
            }
        }))
        .with_flex_spacer(1.)
        .with_child(Button::<AppState>::new("Settings").on_click(|_, data, _| {
            data.current_view = View::Settings;
        }))
        .with_default_spacer()
        .with_child(
            Button::<AppState>::dynamic(|data, _| {
                if data.is_update_available {
                    "⚠️ About".to_string()
                } else {
                    "About".to_string()
                }
            })
            .on_click(|_, data, _| {
                data.current_view = View::About;
            }),
        )
        .padding(10.)
        .background(Color::from_hex_str("#404040").unwrap());

    let view_switcher = ViewSwitcher::<AppState, View>::new(
        |data, _| data.current_view,
        |selector, data, _| match selector {
            View::Instances => Box::new(instances::build_widget()),
            View::InstanceTypeSelection => Box::new(instance_type_selection::build_widget()),
            View::Loading => Box::new(loading::build_widget()),
            View::Progress => Box::new(progress::build_widget()),
            View::InstanceVersionSelection => Box::new(instance_version_selection::build_widget(
                &data.new_instance_state.available_minecraft_versions,
            )),
            View::InstanceNameSelection => Box::new(instance_name_selection::build_widget()),
            View::Accounts => Box::new(accounts::build_widget()),
            View::Runtimes => Box::new(runtimes::build_widget()),
            View::InstallRuntime => {
                Box::new(install_runtime::build_widget(&data.available_runtimes))
            }
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
