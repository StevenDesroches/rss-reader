use crate::error;
use crate::shared::database::{Db, IDb};
use crate::shared::errors::*;

use super::entities::Category;

pub(super) struct CategoryModel {
    db: Db,
}

impl CategoryModel {
    pub fn new() -> Self {
        CategoryModel { db: Db::new() }
    }

    pub fn open(mut self) -> Result<Self> {
        self.db.open()?;
        Ok(self)
    }

    pub fn close(mut self) -> Result<()> {
        self.db.close()
    }

    pub fn insert_category(self, category: Category) -> Result<Self> {
        let connection = self
            .db
            .connection
            .as_ref()
            .ok_or(error!(ErrorType::Model("DB NOT OPEN".to_string())))?;

        connection
            .execute(
                "INSERT INTO category (title, parent_id) VALUES (?1, ?2)",
                (category.title, category.parent_id),
            )
            .map_err(|e| error!(ErrorType::Model(e.to_string())))?;
        // let category_id = connection.last_insert_rowid();
        Ok(self)
    }

    pub fn get_categories(&self) -> Result<Vec<(i32, Option<i32>, String)>> {
        let connection = self
            .db
            .connection
            .as_ref()
            .ok_or(error!(ErrorType::Model("DB NOT OPEN".to_string())))?;

        let mut statement = connection
            .prepare("SELECT * FROM category ORDER BY COALESCE(parent_id, id), title;")
            .map_err(|e| error!(ErrorType::Model(e.to_string())))?;

        let rows = statement
            .query_map([], |row| {
                Ok((
                    row.get::<_, i32>(0)?,
                    row.get::<_, Option<i32>>(1)?,
                    row.get::<_, String>(2)?,
                ))
            })
            .map_err(|e| error!(ErrorType::Model(e.to_string())))?;

        let mut categories: Vec<(i32, Option<i32>, String)> = Vec::new();
        for row in rows {
            categories.push(row.map_err(|e| error!(ErrorType::Model(e.to_string())))?);
        }
        Ok(categories)
    }
}
