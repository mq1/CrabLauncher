// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use color_eyre::Result;
use iced::{
    widget::{button, column, container, horizontal_space, row, scrollable, text, vertical_space},
    Element, Length,
};

use crate::{lib, style, FetchError, Message};

pub struct NewsView {
    pub news: Option<Result<lib::minecraft_news::News, FetchError>>,
}

impl NewsView {
    pub fn new() -> Self {
        Self { news: None }
    }

    pub fn view(&self) -> Element<Message> {
        let heading = text("News").size(50);

        let news: Element<_> = match &self.news {
            Some(Ok(news)) => scrollable(
                column(
                    news.article_grid
                        .iter()
                        .map(|article| {
                            container(
                                row![
                                    text(&article.default_tile.title),
                                    horizontal_space(Length::Fill),
                                    button("Open"),
                                ]
                                .padding(10),
                            )
                            .style(style::card())
                            .into()
                        })
                        .collect(),
                )
                .spacing(10),
            )
            .into(),
            Some(Err(_)) => text("Failed to load news").into(),
            None => text("Loading news...").into(),
        };

        column!(heading, vertical_space(Length::Units(20)), news)
            .padding(20)
            .into()
    }
}
