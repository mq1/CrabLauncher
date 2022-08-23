use druid::{
    widget::{Button, Flex, Label, List, Scroll},
    Widget, WidgetExt,
};

use crate::{lib::minecraft_news::MINECRAFT_NEWS_BASE_URL, AppState};

pub fn build_widget() -> impl Widget<AppState> {
    Scroll::new(
        List::new(|| {
            Flex::row()
                .with_child(Label::new(|(item, _): &(String, String), _env: &_| {
                    item.to_string()
                }))
                .with_flex_spacer(1.)
                .with_child(Button::new("Open").on_click(|_ctx, (_, url), _env: &_| {
                    open::that(format!("{MINECRAFT_NEWS_BASE_URL}{url}")).unwrap();
                }))
        })
        .with_spacing(10.)
        .lens(AppState::news),
    )
    .vertical()
}

pub fn update_news(event_sink: druid::ExtEventSink) {
    let news = crate::lib::minecraft_news::fetch(None).unwrap();
    event_sink.add_idle_callback(move |data: &mut AppState| {
        data.news = news
            .article_grid
            .into_iter()
            .map(|article| (article.default_tile.title, article.article_url))
            .collect();
    });
}
