use rusqlite::Connection;

use super::entities::Feed;
use crate::error::{Error, Result};
use crate::shared::database::{Db, IDb};

pub(super) struct FeedModel {
    db: Db,
}

impl FeedModel {
    pub fn new() -> Self {
        FeedModel { db: Db::new() }
    }

    pub fn setup() {
        let path = "db.sqlite";
        if !std::path::Path::exists(std::path::Path::new(&path)) {
            let connection = Connection::open(&path).expect("error with connection open");

            let queries = std::fs::read_to_string("assets/sql/init.sqlite3-query")
                .expect("sql init file doesn't exists");
            let queries = queries.as_str();

            connection
                .execute_batch(queries)
                .expect("ERROR RUNNING QUERIES");
            connection.close().expect("ERROR CLOSING");
        }
    }

    pub fn open(mut self) -> Result<Self> {
        self.db.open()?;
        Ok(self)
    }

    pub fn close(mut self) -> Result<()> {
        self.db.close()
    }

    pub fn insert_feed(self, feed: Feed) -> Result<Self> {
        let connection = self
            .db
            .connection
            .as_ref()
            .ok_or(Error::Model("DB NOT OPEN".to_string()))?;

        connection
            .execute(
                "INSERT INTO feed (title, xml_url) VALUES (?1, ?2)",
                (feed.title, feed.xml_url),
            )
            .map_err(|e| Error::Model(e.to_string()))?;
        let feed_id = connection.last_insert_rowid();

        {
            let mut article_statement = connection
                .prepare("INSERT INTO article (title, content) VALUES (?1, ?2)")
                .map_err(|e| Error::Model(e.to_string()))?;
            let mut xref_statement = connection
                .prepare("INSERT INTO feed_article_xref (feed_id, article_id) VALUES (?1, ?2)")
                .map_err(|e| Error::Model(e.to_string()))?;

            // let transaction = connection.transaction().map_err(|e| Error::Model(e.to_string()))?;
            connection
                .execute("BEGIN TRANSACTION", [])
                .map_err(|e| Error::Model(e.to_string()))?;

            for feed_article in feed.articles {
                let feed_article_id = article_statement
                    .insert([feed_article.title, feed_article.content])
                    .map_err(|e| Error::Model(e.to_string()))?;
                xref_statement
                    .execute((feed_id, feed_article_id))
                    .map_err(|e| Error::Model(e.to_string()))?;
            }
            connection
                .execute("Commit", [])
                .map_err(|e| Error::Model(e.to_string()))?;
            // transaction.commit().map_err(|e| Error::Model(e.to_string()))?;
        }

        Ok(self)
    }

    pub fn get_feeds(&self) -> Result<Vec<(i32, String, String)>> {
        let connection = self
            .db
            .connection
            .as_ref()
            .ok_or(Error::Model("DB NOT OPEN".to_string()))?;

        let mut statement = connection
            .prepare("SELECT * FROM feed;")
            .map_err(|e| Error::Model(e.to_string()))?;

        let rows = statement
            .query_map([], |row| {
                Ok((
                    row.get::<_, i32>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                ))
            })
            .map_err(|e| Error::Model(e.to_string()))?;

        // let feeds: (id, title, xml_url) = rows.collect()?;
        let mut feeds: Vec<(i32, String, String)> = Vec::new();
        for row in rows {
            feeds.push(row.map_err(|e| Error::Model(e.to_string()))?);
        }
        Ok(feeds)
    }

    pub fn get_articles_for_feed(&self, feed_id: i32) -> Result<Vec<(i32, String, String)>> {
        let connection = self
            .db
            .connection
            .as_ref()
            .ok_or(Error::Model("DB NOT OPEN".to_string()))?;

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
            .map_err(|e| Error::Model(e.to_string()))?;
        let rows = statement
            .query_map([feed_id], |row| {
                Ok((
                    row.get::<_, i32>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                ))
            })
            .map_err(|e| Error::Model(e.to_string()))?;

        let mut articles: Vec<(i32, String, String)> = Vec::new();
        for row in rows {
            articles.push(row.map_err(|e| Error::Model(e.to_string()))?);
        }
        Ok(articles)
    }
}
