// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use druid::{
    widget::{Button, CrossAxisAlignment, Flex, Label, List, Scroll},
    Color, LensExt, Widget, WidgetExt,
};

use crate::{
    lib::minecraft_news::{Article, News, MINECRAFT_NEWS_BASE_URL},
    AppState,
};

pub fn build_widget() -> impl Widget<AppState> {
    let news = Scroll::new(
        List::new(|| {
            Flex::row()
                .with_child(Label::<Article>::dynamic(|article, _| {
                    article.default_tile.title.to_owned()
                }))
                .with_flex_spacer(1.)
                .with_child(Button::<Article>::new("Open ↗️").on_click(|_, article, _| {
                    open::that(format!(
                        "{MINECRAFT_NEWS_BASE_URL}{url}",
                        url = article.article_url
                    ))
                    .expect("Failed to open article in browser");
                }))
                .padding(5.)
                .border(Color::GRAY, 1.)
                .rounded(5.)
        })
        .with_spacing(10.)
        .lens(AppState::news.then(News::article_grid)),
    )
    .vertical();

    Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(Label::new("🌎 News").with_text_size(32.))
        .with_default_spacer()
        .with_flex_child(news, 1.)
        .padding(10.)
}
