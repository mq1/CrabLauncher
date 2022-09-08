// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use druid::{
    widget::{Button, CrossAxisAlignment, Either, Flex, Label, List, Scroll, Spinner},
    Color, UnitPoint, Widget, WidgetExt,
};

use crate::{
    lib::{self, minecraft_news::MINECRAFT_NEWS_BASE_URL},
    AppState,
};

pub fn build_widget() -> impl Widget<AppState> {
    let loading = Flex::column()
        .with_child(Label::new("Loading..."))
        .with_default_spacer()
        .with_child(Spinner::new())
        .align_horizontal(UnitPoint::CENTER)
        .align_vertical(UnitPoint::CENTER);

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

    let either = Either::new(|data, _env| data.news.is_empty(), loading, news);

    Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(Label::new("🌎 News").with_text_size(32.))
        .with_default_spacer()
        .with_flex_child(either, 1.)
        .padding(10.)
}

pub fn update_news(event_sink: druid::ExtEventSink) {
    let news = lib::minecraft_news::fetch(None).unwrap();

    event_sink.add_idle_callback(move |data: &mut AppState| {
        data.news = news
            .article_grid
            .into_iter()
            .map(|article| (article.default_tile.title, article.article_url))
            .collect();
    });
}

fn open_article(url: &String) {
    open::that(format!("{MINECRAFT_NEWS_BASE_URL}{url}")).unwrap();
}
