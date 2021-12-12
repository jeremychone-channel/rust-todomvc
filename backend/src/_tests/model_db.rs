use super::init_db;

#[tokio::test]
async fn model_db_init_db() -> Result<(), Box<dyn std::error::Error>> {
	// ACTION
	let db = init_db().await?;

	// CHECK
	let result = sqlx::query("SELECT * from todo").fetch_all(&db).await?;
	assert_eq!(2, result.len(), "number of seed todos");

	Ok(())
}
