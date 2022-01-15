use crate::model::Db;
use crate::security::{utx_from_token, UserCtx};
use std::convert::Infallible;
use std::sync::Arc;
use warp::{Filter, Rejection};

pub fn todo_rest_filters<F>(
	base_path: &'static str,
	db: Arc<Db>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
	let todos_path = warp::path(base_path).and(warp::path("todos"));
}

// region:    Filter Utils
pub fn with_db(db: Arc<Db>) -> impl Filter<Extract = (Arc<Db>,), Error = Infallible> + Clone {
	warp::any().map(move || db.clone())
}

pub fn do_auth(_db: Arc<Db>) -> impl Filter<Extract = (UserCtx,), Error = Rejection> + Clone {
	warp::any().and_then(|| async { Ok::<UserCtx, Rejection>(utx_from_token("123").await.unwrap()) })
}
// endregion: Filter Utils
