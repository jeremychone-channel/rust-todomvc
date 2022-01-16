use crate::model::{Db, TodoMac};
use crate::security::{utx_from_token, UserCtx};
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
	let common = with_db(db.clone()).and(do_auth(db.clone()));

	// LIST todos `GET todos/`
	let list = todos_path
		.and(warp::get())
		.and(warp::path::end())
		.and(common.clone())
		.and_then(todo_list);

	list
}

async fn todo_list(db: Arc<Db>, utx: UserCtx) -> Result<Json, warp::Rejection> {
	// FIXME: Add proper error handling
	let todos = TodoMac::list(&db, &utx).await.unwrap();

	let response = json!({ "data": todos });
	Ok(warp::reply::json(&response))
}

// region:    Filter Utils
pub fn with_db(db: Arc<Db>) -> impl Filter<Extract = (Arc<Db>,), Error = Infallible> + Clone {
	warp::any().map(move || db.clone())
}

pub fn do_auth(_db: Arc<Db>) -> impl Filter<Extract = (UserCtx,), Error = Rejection> + Clone {
	warp::any().and_then(|| async { Ok::<UserCtx, Rejection>(utx_from_token("123").await.unwrap()) })
}
// endregion: Filter Utils

// region:    Test
#[cfg(test)]
#[path = "../_tests/web_todo.rs"]
mod tests;
// endregion: Test
