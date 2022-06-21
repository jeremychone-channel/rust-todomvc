use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::fs;
use std::path::PathBuf;
use std::time::Duration;

const PG_HOST: &str = "localhost";
const PG_ROOT_DB: &str = "postgres";
const PG_ROOT_USER: &str = "postgres";
const PG_ROOT_PWD: &str = "postgres";
// app db
const PG_APP_DB: &str = "app_db";
const PG_APP_USER: &str = "app_user";
const PG_APP_PWD: &str = "app_pwd_to_change";
const PG_APP_MAX_CON: u32 = 5;
// sql files
const SQL_DIR: &str = "sql/";
const SQL_RECREATE: &str = "sql/00-recreate-db.sql";

pub type Db = Pool<Postgres>;

pub async fn init_db() -> Result<Db, sqlx::Error> {
	// -- Create the db with PG_ROOT (dev only)
	{
		let root_db = new_db_pool(PG_HOST, PG_ROOT_DB, PG_ROOT_USER, PG_ROOT_PWD, 1).await?;
		pexec(&root_db, SQL_RECREATE).await?;
	}

	// -- Run the app sql files
	let app_db = new_db_pool(PG_HOST, PG_APP_DB, PG_APP_USER, PG_APP_PWD, 1).await?;
	let mut paths: Vec<PathBuf> = fs::read_dir(SQL_DIR)?
		.into_iter()
		.filter_map(|e| e.ok().map(|e| e.path()))
		.collect();
	paths.sort();
	// execute each file
	for path in paths {
		if let Some(path) = path.to_str() {
			// only .sql and not the recreate
			if path.ends_with(".sql") && path != SQL_RECREATE {
				pexec(&app_db, &path).await?;
			}
		}
	}

	// returning the app db
	new_db_pool(PG_HOST, PG_APP_DB, PG_APP_USER, PG_APP_PWD, PG_APP_MAX_CON).await
}

async fn pexec(db: &Db, file: &str) -> Result<(), sqlx::Error> {
	// Read the file
	let content = fs::read_to_string(file).map_err(|ex| {
		println!("ERROR reading {} (cause: {:?})", file, ex);
		ex
	})?;

	// TODO: Make the split more sql proof
	let sqls: Vec<&str> = content.split(";").collect();

	for sql in sqls {
		match sqlx::query(&sql).execute(db).await {
			Ok(_) => (),
			Err(ex) => println!("WARNING - pexec - Sql file '{}' FAILED cause: {}", file, ex),
		}
	}

	Ok(())
}

async fn new_db_pool(host: &str, db: &str, user: &str, pwd: &str, max_con: u32) -> Result<Db, sqlx::Error> {
	let con_string = format!("postgres://{}:{}@{}/{}", user, pwd, host, db);
	PgPoolOptions::new()
		.max_connections(max_con)
		.acquire_timeout(Duration::from_millis(500)) // Needs to find replacement
		.connect(&con_string)
		.await
}

// region:    Test
#[cfg(test)]
#[path = "../_tests/model_db.rs"]
mod tests;
// endregion: Test
