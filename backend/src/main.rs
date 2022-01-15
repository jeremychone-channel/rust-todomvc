#![allow(unused)] // silence unused warnings while exploring (to comment out)
use model::init_db;
use std::env;
use std::sync::Arc;

mod model;
mod security;
mod web;

const DEFAULT_WEB_FOLDER: &'static str = "web-folder/";
const DEFAULT_WEB_PORT: u16 = 8080;

#[tokio::main]
async fn main() {
	// compute the web_folder
	let mut args: Vec<String> = env::args().collect();
	let web_folder = args.pop().unwrap_or_else(|| DEFAULT_WEB_FOLDER.to_string());
	let web_port = DEFAULT_WEB_PORT;

	// get the database
	// TODO - loop until valid DB
	let db = init_db().await.expect("Cannot init db");
	let db = Arc::new(db);
}
