use anyhow::Result;
use serde::{Deserialize, Serialize};
use url::Url;
use isahc::{Request, RequestExt, ReadResponseExt};

const MINECRAFT_NEWS_URL: &str = "https://www.minecraft.net/content/minecraft-net/_jcr_content.articles.grid";
pub const MINECRAFT_NEWS_BASE_URL: &str = "https://www.minecraft.net";


#[derive(Serialize, Deserialize)]
pub struct News {
    pub article_grid: Vec<Article>,
    pub article_count: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Article {
    pub default_tile: Tile,
    #[serde(rename = "articleLang")]
    pub article_lang: ArticleLang,
    pub primary_category: String,
    pub categories: Vec<String>,
    pub article_url: String,
    pub publish_date: String,
    pub tags: Vec<String>,
    pub preferred_tile: Option<Tile>,
}

#[derive(Serialize, Deserialize)]
pub struct Tile {
    pub sub_header: String,
    pub image: Image,
    pub tile_size: TileSize,
    pub title: String,
}

#[derive(Serialize, Deserialize)]
pub struct Image {
    pub content_type: ContentType,
    #[serde(rename = "imageURL")]
    pub image_url: String,
    pub alt: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub enum ArticleLang {
    #[serde(rename = "en-us")]
    EnUs,
}

#[derive(Serialize, Deserialize)]
pub enum ContentType {
    #[serde(rename = "image")]
    Image,
}

#[derive(Serialize, Deserialize)]
pub enum TileSize {
    #[serde(rename = "1x1")]
    The1X1,
    #[serde(rename = "2x1")]
    The2X1,
    #[serde(rename = "2x2")]
    The2X2,
}

/// Get the news from minecraft.net
pub fn fetch(page_size: Option<usize>) -> Result<News> {
    let page_size = page_size.unwrap_or(20);

    let url = Url::parse_with_params(MINECRAFT_NEWS_URL, &[("pageSize", page_size.to_string())])?;
    let user_agent = format!("{}/{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));

    let news = Request::get(url.to_string())
        .header("user-agent", &user_agent)
        .body(())?
        .send()?
        .json()?;

    Ok(news)
}
