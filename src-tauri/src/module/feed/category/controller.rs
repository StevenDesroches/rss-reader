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
            Some(parent_id) => Category::builder().title(title).parent(parent_id).build(),
            None => Category::builder().title(title).build(),
        };
        CategoryModel::new()
            .open()?
            .insert_category(category)?
            .close()?;
        Ok(())
    }

    pub fn get_all(&self) -> Result<Vec<Category>> {
        let model = CategoryModel::new().open()?;
        let mut categories = Vec::new();

        let model_categories = model.get_categories()?;
        model.close()?;
        let _: Vec<_> = model_categories
            .into_iter()
            .map(|i| match i.1 {
                Some(parent_id) => {
                    categories.push(Category::builder().id(i.0).parent(parent_id).title(i.2).build())
                }
                None => categories.push(Category::builder().id(i.0).title(i.2).build()),
            })
            .collect();
        Ok(categories)
    }
}
