use crate::error::{Error, Result};
use crate::shared::database::{Db, IDb};

use super::entities::Category;

pub struct CategoryModel {
    db: Db,
}

impl CategoryModel {
    pub fn new() -> Self {
        CategoryModel { db: Db::new() }
    }

    pub fn open(mut self) -> Self {
        self.db.open();
        self
    }

    pub fn close(mut self) -> Result<()> {
        self.db.close()
    }

    pub fn insert_category(self, category: Category) -> Result<Self> {
        let connection = self
            .db
            .connection
            .as_ref()
            .ok_or(Error::Custom("DB NOT OPEN".to_string()))?;

        connection
            .execute(
                "INSERT INTO category (title, parent_id) VALUES (?1, ?2)",
                (category.title, category.parent_id),
            )
            .map_err(|e| Error::Custom(e.to_string()))?;
        // let category_id = connection.last_insert_rowid();
        Ok(self)
    }
}
