// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use std::thread;

use druid::{
    widget::{Button, Flex},
    Color, Widget, WidgetExt,
};

use crate::{lib, AppState, View};

pub fn build_widget() -> impl Widget<AppState> {
    Flex::column()
        .with_child(Button::<AppState>::new("Instances").on_click(|_, data, _| {
            data.current_view = View::Instances;
        }))
        .with_default_spacer()
        .with_child(Button::<AppState>::new("News").on_click(|ctx, data, _| {
            if data.news.article_count == 0 {
                let event_sink = ctx.get_external_handle();
                thread::spawn(move || lib::minecraft_news::update_news(event_sink));
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
        .background(Color::from_hex_str("#404040").unwrap())
}
