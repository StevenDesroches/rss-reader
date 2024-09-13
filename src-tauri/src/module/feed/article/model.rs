use crate::error;
use crate::shared::database::{Db, IDb};
use crate::shared::errors::*;

use super::entities::Article;

pub struct ArticleModel {
    db: Db,
}

impl ArticleModel {
    pub fn new() -> Self {
        ArticleModel { db: Db::new() }
    }

    pub fn open(mut self) -> Result<Self> {
        self.db.open()?;
        Ok(self)
    }

    pub fn close(mut self) -> Result<()> {
        self.db.close()
    }

    pub fn _insert_article(self, article: Article) -> Result<Self> {
        let connection = self
            .db
            .connection
            .as_ref()
            .ok_or(error!(ErrorType::Model("DB NOT OPEN".to_string())))?;

        connection
            .execute(
                "INSERT INTO article (title, content) VALUES (?1, ?2)",
                (article.title, article.content),
            )
            .map_err(|e| error!(ErrorType::Model(e.to_string())))?;
        // let article_id = connection.last_insert_rowid();
        Ok(self)
    }

    pub fn insert_articles(self, feed_id: i64, articles: &Vec<Article>) -> Result<Self> {
        let connection = self
            .db
            .connection
            .as_ref()
            .ok_or(error!(ErrorType::Model("DB NOT OPEN".to_string())))?;

        {
            let mut article_statement = connection
                .prepare("INSERT INTO article (title, content) VALUES (?1, ?2)")
                .map_err(|e| error!(ErrorType::Model(e.to_string())))?;
            let mut xref_statement = connection
                .prepare("INSERT INTO feed_article_xref (feed_id, article_id) VALUES (?1, ?2)")
                .map_err(|e| error!(ErrorType::Model(e.to_string())))?;

            // let transaction = connection.transaction().map_err(|e| Error::Model(e.to_string()))?;
            connection
                .execute("BEGIN TRANSACTION", [])
                .map_err(|e| error!(ErrorType::Model(e.to_string())))?;

            for article in articles {
                let article_id = article_statement
                    .insert([&article.title, &article.content])
                    .map_err(|e| error!(ErrorType::Model(e.to_string())))?;
                xref_statement
                    .execute((feed_id, article_id))
                    .map_err(|e| error!(ErrorType::Model(e.to_string())))?;
            }
            connection
                .execute("Commit", [])
                .map_err(|e| error!(ErrorType::Model(e.to_string())))?;
            // transaction.commit().map_err(|e| Error::Model(e.to_string()))?;
        }

        Ok(self)
    }

    pub fn get_articles_by_feed(&self, feed_id: i32) -> Result<Vec<(i32, String, String)>> {
        let connection = self
            .db
            .connection
            .as_ref()
            .ok_or(error!(ErrorType::Model("DB NOT OPEN".to_string())))?;

        let mut statement = connection
            .prepare(
                "SELECT
                        *
                    FROM
                        article
                        LEFT JOIN feed_article_xref as xref ON xref.article_id = article.id
                    WHERE
                        xref.feed_id = ?1
                    ORDER BY
                        `id` ASC
                    ",
            )
            .map_err(|e| error!(ErrorType::Model(e.to_string())))?;
        let rows = statement
            .query_map([feed_id], |row| {
                Ok((
                    row.get::<_, i32>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                ))
            })
            .map_err(|e| error!(ErrorType::Model(e.to_string())))?;

        let mut articles: Vec<(i32, String, String)> = Vec::new();
        for row in rows {
            articles.push(row.map_err(|e| error!(ErrorType::Model(e.to_string())))?);
        }
        Ok(articles)
    }
}
