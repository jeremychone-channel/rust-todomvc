mod db;
mod todo;

// region:    Error
#[derive(thiserror::Error, Debug)]
pub enum Error {
	#[error("Entity Not Found - {0}[{1}] ")]
	EntityNotFound(&'static str, String),

	#[error(transparent)]
	SqlxError(#[from] sqlx::Error),

	#[error(transparent)]
	IOError(#[from] std::io::Error),
}

// endregion: Error
