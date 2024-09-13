// use crate::error;
use crate::shared::errors::*;

use super::{controller::ArticleController, entities::Article};

#[tauri::command]
pub async fn get_articles_by_feed(feed_id: i32) -> Result<Vec<Article>> {
    let articles = ArticleController::get_articles_by_feed(feed_id)?;
    Ok(articles)
}
