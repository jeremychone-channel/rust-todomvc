use super::filter_auth::do_auth;
use crate::model::{Db, TodoMac, TodoPatch};
use crate::security::{utx_from_token, UserCtx};
use serde::Serialize;
use serde_json::json;
use std::convert::Infallible;
use std::sync::Arc;
use warp::reply::Json;
use warp::{Filter, Rejection};

pub fn todo_rest_filters(
	base_path: &'static str,
	db: Arc<Db>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	let todos_path = warp::path(base_path).and(warp::path("todos"));
	let common = super::filter_utils::with_db(db.clone()).and(do_auth(db.clone()));

	// LIST todos `GET todos/`
	let list = todos_path
		.and(warp::get())
		.and(warp::path::end())
		.and(common.clone())
		.and_then(todo_list);

	/// GET todo `GET /todos/100`
	let get = todos_path
		.and(warp::get())
		.and(common.clone())
		.and(warp::path::param())
		.and_then(todo_get);

	/// CREATE todo `POST /todos with body TodoPatch`
	let create = todos_path
		.and(warp::post())
		.and(common.clone())
		.and(warp::body::json())
		.and_then(todo_create);

	list
}

async fn todo_list(db: Arc<Db>, utx: UserCtx) -> Result<Json, warp::Rejection> {
	let todos = TodoMac::list(&db, &utx).await?;
	json_response(todos)
}

async fn todo_get(db: Arc<Db>, utx: UserCtx, id: i64) -> Result<Json, warp::Rejection> {
	let todo = TodoMac::get(&db, &utx, id).await?;
	json_response(todo)
}

async fn todo_create(db: Arc<Db>, utx: UserCtx, patch: TodoPatch) -> Result<Json, warp::Rejection> {
	let todo = TodoMac::create(&db, &utx, patch).await?;
	json_response(todo)
}

async fn todo_update(db: Arc<Db>, utx: UserCtx, id: i64, patch: TodoPatch) -> Result<Json, warp::Rejection> {
	let todo = TodoMac::update(&db, &utx, id, patch).await?;
	json_response(todo)
}

async fn todo_delete(db: Arc<Db>, utx: UserCtx, id: i64) -> Result<Json, warp::Rejection> {
	let todo = TodoMac::delete(&db, &utx, id).await?;
	json_response(todo)
}

// region:    Utils
fn json_response<D: Serialize>(data: D) -> Result<Json, warp::Rejection> {
	let response = json!({ "data": data });
	Ok(warp::reply::json(&response))
}
// endregion: Utils

// region:    Test
#[cfg(test)]
#[path = "../_tests/web_todo.rs"]
mod tests;
// endregion: Test
