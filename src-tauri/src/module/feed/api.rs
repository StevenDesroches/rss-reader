use super::controller::FeedController;
use super::entities::FeedArticle;
use super::entities::Feed;

use crate::shared::types::Url;
use crate::error::Result;

use serde_json::json;
use serde_json::Value;

#[tauri::command]
pub async fn fetch_feed(url: Url) -> Result<Value> {
    let feed = FeedController::fetch(url).await?;
    Ok(json!(feed))
}

#[tauri::command]
pub async fn add_feed(url: Url, title: String, category_id: Option<i32>) -> Result<()> {
    let feed_controller = FeedController::new();
    feed_controller.add(url, title, category_id).await?;
    Ok(())
}

#[tauri::command]
pub async fn get_all_feeds() -> Result<Vec<Feed>> {
    let feeds = FeedController::new().get_all()?;
    Ok(feeds)
}

#[tauri::command]
pub async fn get_articles_for_feed(feed_id: i32) -> Result<Vec<FeedArticle>> {
    let articles = FeedController::new().get_articles(feed_id)?;
    Ok(articles)
}
