use super::filter_utils::with_db;
use crate::model::Db;
use crate::security::{utx_from_token, UserCtx};
use crate::web::Error;
use std::sync::Arc;
use warp::{Filter, Rejection};

const HEADER_XAUTH: &str = "X-Auth-Token";

pub fn do_auth(db: Arc<Db>) -> impl Filter<Extract = (UserCtx,), Error = Rejection> + Clone {
	warp::any()
		.and(with_db(db))
		.and(warp::header::optional(HEADER_XAUTH))
		.and_then(|db: Arc<Db>, xauth: Option<String>| async move {
			match xauth {
				Some(xauth) => {
					let utx = utx_from_token(&db, &xauth).await?;
					Ok::<UserCtx, Rejection>(utx)
				}
				None => Err(Error::FailAuthMissingXAuth.into()),
			}
		})
}
