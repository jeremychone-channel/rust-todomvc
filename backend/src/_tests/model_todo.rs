use crate::model::db::init_db;
use crate::model::todo::{TodoPatch, TodoStatus};

use super::TodoMac;

#[tokio::test]
async fn model_todo_create() -> Result<(), Box<dyn std::error::Error>> {
	// -- FIXTURE
	let db = init_db().await?;
	let data_fx = TodoPatch {
		title: Some("test - model_todo_create 1".to_string()),
		..Default::default()
	};

	// -- ACTION
	let todo_created = TodoMac::create(&db, data_fx.clone()).await?;

	// -- CHECK
	assert!(todo_created.id >= 1000, "Id should be >= 1000");
	assert_eq!(data_fx.title.unwrap(), todo_created.title);
	assert_eq!(TodoStatus::Open, todo_created.status);

	Ok(())
}

#[tokio::test]
async fn model_todo_list() -> Result<(), Box<dyn std::error::Error>> {
	// -- FIXTURE
	let db = init_db().await?;

	// -- ACTION
	let todos = TodoMac::list(&db).await?;

	// -- CHECK
	assert_eq!(2, todos.len());
	// todo 101
	assert_eq!(101, todos[0].id);
	assert_eq!(123, todos[0].cid);
	assert_eq!("todo 101", todos[0].title);
	// todo 100
	assert_eq!(100, todos[1].id);
	assert_eq!(123, todos[1].cid);
	assert_eq!("todo 100", todos[1].title);

	Ok(())
}
