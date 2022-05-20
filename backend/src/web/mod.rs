use crate::model::{self, Db};
use crate::security;
use crate::web::todo::todo_rest_filters;
use serde_json::json;
use std::convert::Infallible;
use std::path::Path;
use std::sync::Arc;
use warp::{Filter, Rejection, Reply};

mod filter_auth;
mod filter_utils;
mod todo;

pub async fn start_web(web_folder: &str, web_port: u16, db: Arc<Db>) -> Result<(), Error> {
	// validate web_folder
	if !Path::new(web_folder).exists() {
		return Err(Error::FailStartWebFolderNotFound(web_folder.to_string()));
	}

	println!("...1");
	// Apis
	let apis = todo_rest_filters("api", db);

	// Static content
	let content = warp::fs::dir(web_folder.to_string());
	let root_index = warp::get()
		.and(warp::path::end())
		.and(warp::fs::file(format!("{}/index.html", web_folder)));
	let static_site = content.or(root_index);

	// Combine all routes
	let routes = apis.or(static_site).recover(handle_rejection);

	println!("Start 127.0.0.1:{} at {}", web_port, web_folder);
	warp::serve(routes).run(([127, 0, 0, 1], web_port)).await;

	Ok(())
}

async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
	// Print to server side
	println!("ERROR - {:?}", err);

	// TODO - Call log API for capture and store

	// Build user message
	let user_message = match err.find::<WebErrorMessage>() {
		Some(err) => err.typ.to_string(),
		None => "Unknown".to_string(),
	};

	let result = json!({ "errorMessage": user_message });
	let result = warp::reply::json(&result);

	Ok(warp::reply::with_status(result, warp::http::StatusCode::BAD_REQUEST))
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
	#[error("Web server failed to start because web-folder '{0}' not found.")]
	FailStartWebFolderNotFound(String),

	#[error("Fail authentication missing X-Auth-Token header.")]
	FailAuthMissingXAuth,
}

// region:    Warp Custom Error
#[derive(Debug)]
pub struct WebErrorMessage {
	pub typ: &'static str,
	pub message: String,
}
impl warp::reject::Reject for WebErrorMessage {}

impl WebErrorMessage {
	pub fn rejection(typ: &'static str, message: String) -> warp::Rejection {
		warp::reject::custom(WebErrorMessage { typ, message })
	}
}

impl From<self::Error> for warp::Rejection {
	fn from(other: self::Error) -> Self {
		WebErrorMessage::rejection("web::Error", format!("{}", other))
	}
}
impl From<model::Error> for warp::Rejection {
	fn from(other: model::Error) -> Self {
		WebErrorMessage::rejection("model::Error", format!("{}", other))
	}
}
impl From<security::Error> for warp::Rejection {
	fn from(other: security::Error) -> Self {
		WebErrorMessage::rejection("security::Error", format!("{}", other))
	}
}
// endregion: Warp Custom Error
