use crate::error::Result;

use super::entities::Category;
use super::model::CategoryModel;

pub struct CategoryController {}

impl CategoryController {
    pub fn new() -> Self {
        CategoryController {}
    }

    pub async fn add(&self, title: String, parent_id: Option<i32>) -> Result<()> {
        let category = match parent_id {
            Some(parent_id) =>  Category::builder().title(title).parent(parent_id).build(),
            None => Category::builder().title(title).build(),
        };
        CategoryModel::new().open().insert_category(category)?.close()?;
        Ok(())
    }

}
