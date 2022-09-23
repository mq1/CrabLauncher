// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use druid::{
    widget::{Button, CrossAxisAlignment, Flex, Label, List, Scroll},
    Color, Widget, WidgetExt,
};

use crate::{lib::minecraft_news::MINECRAFT_NEWS_BASE_URL, AppState};

pub fn build_widget() -> impl Widget<AppState> {
    let news = Scroll::new(
        List::new(|| {
            Flex::row()
                .with_child(Label::new(|(item, _): &(String, String), _env: &_| {
                    item.to_string()
                }))
                .with_flex_spacer(1.)
                .with_child(Button::new("Open ↗️").on_click(|_ctx, (_, url), _env: &_| {
                    open_article(url);
                }))
                .padding(5.)
                .border(Color::GRAY, 1.)
                .rounded(5.)
        })
        .with_spacing(10.)
        .lens(AppState::news),
    )
    .vertical();

    Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(Label::new("🌎 News").with_text_size(32.))
        .with_default_spacer()
        .with_flex_child(news, 1.)
        .padding(10.)
}

fn open_article(url: &String) {
    open::that(format!("{MINECRAFT_NEWS_BASE_URL}{url}")).unwrap();
}
