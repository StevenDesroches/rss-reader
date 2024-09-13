use super::{controller::ArticleController, entities::Article};

use crate::error::Result;


#[tauri::command]
pub async fn get_articles_by_feed(feed_id: i32) -> Result<Vec<Article>> {
    let articles = ArticleController::get_articles_by_feed(feed_id)?;
    Ok(articles)
}
