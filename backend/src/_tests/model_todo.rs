use crate::model::db::init_db;

use super::TodoMac;

#[tokio::test]
async fn model_todo_list() -> Result<(), Box<dyn std::error::Error>> {
	// -- FIXTURE
	let db = init_db().await?;

	// -- ACTION
	let todos = TodoMac::list(&db).await?;

	// -- CHECK
	assert_eq!(2, todos.len());

	Ok(())
}
