// SPDX-FileCopyrightText: 2022-present Manuel Quarneti <hi@mq1.eu>
// SPDX-License-Identifier: GPL-3.0-only

use color_eyre::eyre::Result;
use druid::{im::Vector, Data, Lens};
use serde::Deserialize;
use url::Url;

use crate::{AppState, View};

use super::HTTP_CLIENT;

const MINECRAFT_NEWS_URL: &str =
    "https://www.minecraft.net/content/minecraft-net/_jcr_content.articles.grid";
pub const MINECRAFT_NEWS_BASE_URL: &str = "https://www.minecraft.net";

#[derive(Deserialize, Data, Clone, Default, Lens)]
pub struct News {
    pub article_grid: Vector<Article>,
    pub article_count: i32,
}

#[derive(Deserialize, Data, Clone)]
pub struct Article {
    pub default_tile: Tile,
    #[serde(rename = "articleLang")]
    pub article_lang: ArticleLang,
    pub primary_category: String,
    pub categories: Vector<String>,
    pub article_url: String,
    pub publish_date: String,
    pub tags: Vector<String>,
    pub preferred_tile: Option<Tile>,
}

#[derive(Deserialize, Data, Clone)]
pub struct Tile {
    pub sub_header: String,
    pub image: Image,
    pub tile_size: TileSize,
    pub title: String,
}

#[derive(Deserialize, Data, Clone)]
pub struct Image {
    pub content_type: ContentType,
    #[serde(rename = "videoURL")]
    pub video_url: Option<String>,
    #[serde(rename = "videoType")]
    pub video_type: Option<String>,
    #[serde(rename = "imageURL")]
    pub image_url: String,
    #[serde(rename = "videoProvider")]
    pub video_provider: Option<String>,
    pub alt: Option<String>,
    #[serde(rename = "videoId")]
    pub video_id: Option<String>,
    #[serde(rename = "linkurl")]
    pub link_url: Option<String>,
    pub background_color: Option<BackgroundColor>,
}

#[derive(Deserialize, Data, Clone, PartialEq, Eq)]
pub enum ArticleLang {
    #[serde(rename = "en-us")]
    EnUs,
}

#[derive(Deserialize, Data, Clone, PartialEq, Eq)]
pub enum BackgroundColor {
    #[serde(rename = "bg-blue")]
    BgBlue,
    #[serde(rename = "bg-green")]
    BgGreen,
    #[serde(rename = "bg-red")]
    BgRed,
}

#[derive(Deserialize, Data, Clone, PartialEq, Eq)]
pub enum ContentType {
    #[serde(rename = "image")]
    Image,
    #[serde(rename = "outgoing-link")]
    OutgoingLink,
    #[serde(rename = "video")]
    Video,
}

#[derive(Deserialize, Data, Clone, PartialEq, Eq)]
pub enum TileSize {
    #[serde(rename = "1x1")]
    The1X1,
    #[serde(rename = "1x2")]
    The1X2,
    #[serde(rename = "2x1")]
    The2X1,
    #[serde(rename = "2x2")]
    The2X2,
    #[serde(rename = "4x2")]
    The4X2,
}

/// Get the news from minecraft.net
fn fetch(page_size: Option<i32>) -> Result<News> {
    let page_size = page_size.unwrap_or(20);
    let url = Url::parse_with_params(MINECRAFT_NEWS_URL, &[("pageSize", page_size.to_string())])?;
    let resp = HTTP_CLIENT.get(&url.to_string()).send()?.json()?;

    Ok(resp)
}

pub fn update_news(event_sink: druid::ExtEventSink) -> Result<()> {
    event_sink.add_idle_callback(move |data: &mut AppState| {
        data.current_message = "Loading news...".to_string();
        data.current_view = View::Loading;
    });

    let news = fetch(None)?;

    event_sink.add_idle_callback(move |data: &mut AppState| {
        data.news = news;
        data.current_view = View::News;
    });

    Ok(())
}
