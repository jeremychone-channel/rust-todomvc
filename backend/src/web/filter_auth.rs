use crate::model::Db;
use crate::security::{utx_from_token, UserCtx};
use std::sync::Arc;
use warp::{Filter, Rejection};

pub fn do_auth(_db: Arc<Db>) -> impl Filter<Extract = (UserCtx,), Error = Rejection> + Clone {
	warp::any().and_then(|| async { Ok::<UserCtx, Rejection>(utx_from_token("123").await?) })
}
