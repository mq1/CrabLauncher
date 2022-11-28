// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use anyhow::Result;
use iced::{
    widget::{button, column, container, horizontal_space, row, scrollable, text},
    Element, Length,
};

use crate::{util, style, Message};

pub struct News {
    pub news: Option<Result<util::minecraft_news::News, String>>,
}

impl News {
    pub fn new() -> Self {
        Self { news: None }
    }

    pub async fn fetch() -> Result<util::minecraft_news::News, String> {
        util::minecraft_news::fetch(None).map_err(|e| e.to_string())
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
                                    button("Open").on_press(Message::OpenURL(article.get_url())),
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
            Some(Err(e)) => text(format!("Error: {}", e)).into(),
            None => text("Loading news...").into(),
        };

        column![heading, news].spacing(20).padding(20).into()
    }
}
