use super::todo_rest_filters;
use crate::model::{init_db, Todo, TodoStatus};
use crate::web::handle_rejection;
use anyhow::{Context, Result};
use serde::Deserialize;
use serde_json::{from_str, from_value, Value};
use std::str::from_utf8;
use std::sync::Arc;
use warp::hyper::body::Bytes;
use warp::hyper::Response;
use warp::Filter;

#[tokio::test]
async fn web_todo_list() -> Result<()> {
	// -- FIXTURE
	let db = init_db().await?;
	let db = Arc::new(db);
	let todo_apis = todo_rest_filters("api", db.clone()).recover(handle_rejection);

	// -- ACTION
	let resp = warp::test::request()
		.method("GET")
		// .header("X-Auth-Token", "123")
		.path("/api/todos")
		.reply(&todo_apis)
		.await;

	// -- CHECK
	assert_eq!(200, resp.status(), "http status");

	// extract response .data
	let todos: Vec<Todo> = extract_body_data(resp)?;

	// -- CHECK - todos
	assert_eq!(2, todos.len(), "number of todos");
	assert_eq!(101, todos[0].id);
	assert_eq!("todo 101", todos[0].title);
	assert_eq!(TodoStatus::Open, todos[0].status);

	Ok(())
}

// region:    Web Test Utils
fn extract_body_data<D>(resp: Response<Bytes>) -> Result<D>
where
	for<'de> D: Deserialize<'de>,
{
	// parse the body as serde_json::Value
	let body = from_utf8(resp.body())?;
	let mut body: Value =
		from_str(body).with_context(|| format!("Cannot parse resp.body to JSON. resp.body: '{}'", body))?;

	// extract the data
	let data = body["data"].take();

	// deserialize the data to D
	let data: D = from_value(data)?;

	Ok(data)
}
// endregion: Web Test Utils
