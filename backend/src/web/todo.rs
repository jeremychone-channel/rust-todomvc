use crate::model::Db;
use std::sync::Arc;
use warp::Filter;

pub fn todo_rest_filters(
	base_path: &'static str,
	db: Arc<Db>,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
}
