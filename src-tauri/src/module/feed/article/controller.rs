use crate::error::Result;

use super::{entities::Article, model::ArticleModel};
pub(super) struct ArticleController {}

impl ArticleController {
    pub fn _new() -> Self {
        ArticleController {}
    }

    // pub fn get_articles() {}
    pub fn get_articles_by_feed(feed_id: i32) -> Result<Vec<Article>> {
        let mut articles = Vec::new();
        let model_articles = ArticleModel::new().open()?.get_articles_by_feed(feed_id)?;

        let _: Vec<_> = model_articles
            .into_iter()
            .map(|i| {
                articles.push(
                    Article::builder()
                        .id(i.0)
                        .title(i.1)
                        .content(i.2)
                        .feed_id(vec![feed_id])
                        .build(),
                )
            })
            .collect();

        Ok(articles)
    }
}
