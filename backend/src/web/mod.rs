use crate::model::Db;
use std::path::Path;
use std::sync::Arc;

pub async fn start_web(web_folder: &str, web_port: u16, db: Arc<Db>) -> Result<(), Error> {
	// validate web_folder
	if !Path::new(web_folder).exists() {
		return Err(Error::FailStartWebFolderNotFound(web_folder.to_string()));
	}

	// Static content

	Ok(())
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
	#[error("Web server failed to start because web-folder '{0}' not found.")]
	FailStartWebFolderNotFound(String),
}
