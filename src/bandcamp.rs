use reqwest::Client;
use scraper::{Html, Selector};
use std::error::Error;

use crate::Album;

pub async fn scrape_album(url: &str) -> Result<Album, Box<dyn Error>> {
    let client = Client::new();
    let res = client.get(url).send().await?.text().await?;

    let document = Html::parse_document(&res);

    // Selectors for artist and cover art
    let _artist_selector = Selector::parse("#name-section h3 span a").unwrap(); // TODO use source
    let image_selector = Selector::parse("meta[property='og:image']").unwrap();
    let page_title_selector = Selector::parse("title").unwrap();

    // Fallback: parse title from <title> tag like "Album | Artist"
    let (album_name, artist_name) = document
        .select(&page_title_selector)
        .next()
        .map(|el| el.text().collect::<String>())
        .and_then(|text| {
            let parts: Vec<&str> = text.split("|").map(|s| s.trim()).collect();
            if parts.len() >= 2 {
                Some((parts[0].to_string(), parts[1].to_string()))
            } else if parts.len() == 1 {
                Some((parts[0].to_string(), parts[0].to_string()))
            } else {
                None
            }
        })
        .unwrap_or(("Unknown Album".to_string(), "Unknown Artist".to_string()));

    let album_url = document
        .select(&image_selector)
        .next()
        .and_then(|el| el.value().attr("content"))
        .unwrap_or("No Image Found")
        .to_string();

    Ok(Album {
        album_name,
        artist_name,
        album_url,
    })
}
