use super::entities::Feed;

use crate::error;
use crate::shared::errors::*;
use crate::shared::database::{Db, IDb};

pub(super) struct FeedModel {
    db: Db,
}

type DbFeeds = Vec<(i32, String, String, Option<i32>)>;

impl FeedModel {
    pub fn new() -> Self {
        FeedModel { db: Db::new() }
    }

    pub fn open(mut self) -> Result<Self> {
        self.db.open()?;
        Ok(self)
    }

    pub fn close(mut self) -> Result<()> {
        self.db.close()
    }

    pub fn insert_feed(&self, feed: &Feed) -> Result<i64> {
        let connection = self
            .db
            .connection
            .as_ref()
            .ok_or(error!(ErrorType::Model("DB NOT OPEN".to_string())))?;

        connection
            .execute(
                "INSERT INTO feed (title, xml_url) VALUES (?1, ?2)",
                (&feed.title, &feed.xml_url),
            )
            .map_err(|e| error!(ErrorType::Model(e.to_string())))?;
        let feed_id = connection.last_insert_rowid();

        if feed.category_id.is_some() {
            connection
                .execute(
                    "INSERT INTO feed_category_xref (feed_id, category_id) VALUES (?1, ?2)",
                    (feed_id, feed.category_id),
                )
                .map_err(|e| error!(ErrorType::Model(e.to_string())))?;
        }
        Ok(feed_id)
    }

    pub fn get_feeds(&self) -> Result<DbFeeds> {
        let connection = self
            .db
            .connection
            .as_ref()
            .ok_or(error!(ErrorType::Model("DB NOT OPEN".to_string())))?;

        let mut statement = connection
            .prepare("SELECT feed.*, xref.category_id FROM feed LEFT JOIN feed_category_xref xref on xref.feed_id = feed.id;")
            .map_err(|e| error!(ErrorType::Model(e.to_string())))?;

        let rows = statement
            .query_map([], |row| {
                Ok((
                    row.get::<_, i32>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, Option<i32>>(3)?,
                ))
            })
            .map_err(|e| error!(ErrorType::Model(e.to_string())))?;

        // let feeds: (id, title, xml_url) = rows.collect()?;
        let mut feeds: DbFeeds = Vec::new();
        for row in rows {
            feeds.push(row.map_err(|e| error!(ErrorType::Model(e.to_string())))?);
        }
        Ok(feeds)
    }
}
