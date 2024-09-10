use crate::error::Result;

use super::controller::CategoryController;

#[tauri::command]
pub async fn add_category(title: String, parent_id: Option<i32>) -> Result<()> {
    CategoryController::new().add(title, parent_id).await?;
    Ok(())
}
