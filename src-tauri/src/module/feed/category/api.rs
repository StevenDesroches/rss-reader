// use crate::error;
use crate::shared::errors::*;

use super::controller::CategoryController;
use super::entities::Category;

#[tauri::command]
pub async fn add_category(title: String, parent_id: Option<i32>) -> Result<()> {
    CategoryController::new().add(title, parent_id).await?;
    Ok(())
}

#[tauri::command]
pub async fn get_all_categories() -> Result<Vec<Category>> {
    let categories = CategoryController::new().get_all()?;
    Ok(categories)
}