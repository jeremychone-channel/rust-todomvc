use std::sync::Arc;

use super::todo_rest_filters;
use crate::model::init_db;
use anyhow::Result;

#[tokio::test]
async fn web_todo_list() -> Result<()> {
	// -- FIXTURE
	let db = init_db().await?;
	let db = Arc::new(db);
	let todo_apis = todo_rest_filters("api", db.clone());

	// -- ACTION
	let resp = warp::test::request()
		.method("GET")
		.path("/api/todos")
		.reply(&todo_apis)
		.await;

	// -- CHECK
	assert_eq!(200, resp.status(), "http status");

	Ok(())
}
