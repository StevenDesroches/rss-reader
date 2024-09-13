// use crate::error;
use crate::shared::errors::*;
use crate::shared::types::Url;

use super::article::model::ArticleModel;
use super::entities::Feed;
use super::model::FeedModel;
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

        let feed_model = FeedModel::new().open()?;
        let feed_id = feed_model.insert_feed(&feed)?;
        feed_model.close()?;

        ArticleModel::new()
            .open()?
            .insert_articles(feed_id, &feed.articles)?
            .close()?;

        Ok(())
    }

    pub fn get_all(&self) -> Result<Vec<Feed>> {
        let model = FeedModel::new().open()?;
        let mut feeds = Vec::new();

        let model_feeds = model.get_feeds()?;
        model.close()?;

        let _: Vec<_> = model_feeds
            .into_iter()
            .map(|i| {
                let feed = match i.3 {
                    Some(category_id) => Feed::builder()
                        .id(i.0)
                        .title(i.1)
                        .xml_url(i.2)
                        .category_id(category_id)
                        .build(),
                    None => Feed::builder().id(i.0).title(i.1).xml_url(i.2).build(),
                };
                feeds.push(feed);
            })
            .collect();

        Ok(feeds)
    }
}
