use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use std::env;

pub async fn get_pool() -> Result<SqlitePool, Box<dyn std::error::Error>> {
	let pool = SqlitePoolOptions::new()
		.max_connections(20)
		.connect(&env::var("DATABASE_URL")?)
		.await?;

	debug!(
		"connected to database at url '{}'",
		&env::var("DATABASE_URL")?
	);

	Ok(pool)
}
