use crate::error::Result;

use super::entities::{Feed, FeedArticle};
use super::model::FeedModel;
use crate::shared::types::Url;
pub(super) struct FeedController {}

impl FeedController {
    pub fn new() -> Self {
        FeedController {}
    }

    ///Get feed without saving it to the DB
    pub async fn fetch(url: Url) -> Result<Feed> {
        Feed::from_url(url).await
    }

    ///Add feed to db
    pub async fn add(&self, url: Url, title: String, category_id: Option<i32>) -> Result<()> {
        let mut feed = Feed::from_url(url).await?;
        feed.title = title;

        if let Some(category_id) = category_id {
            feed.category_id = Some(category_id)
        }

        FeedModel::new().open()?.insert_feed(feed)?.close()?;
        Ok(())
    }

    pub fn get_all(&self) -> Result<Vec<Feed>> {
        let model = FeedModel::new().open()?;
        let mut feeds = Vec::new();

        let model_feeds = model.get_feeds()?;
        model.close()?;
        let _: Vec<_> = model_feeds
            .into_iter()
            .map(|i| feeds.push(Feed::from_model(i.0, i.1, i.2)))
            .collect();

        Ok(feeds)
    }

    pub fn get_articles(&self, feed_id: i32) -> Result<Vec<FeedArticle>> {
        let model = FeedModel::new().open()?;
        let mut articles = Vec::new();
        let model_articles = model.get_articles_for_feed(feed_id)?;
        model.close()?;

        let _: Vec<_> = model_articles
            .into_iter()
            .map(|i| articles.push(FeedArticle::from_model(i.0, i.1, i.2)))
            .collect();

        Ok(articles)
    }
}
