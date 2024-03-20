
use sqlx::sqlite::SqlitePool;
use sqlx::Connection;

async fn create_db(sql_file_location: &str) -> Result<()> {
    let conn = SqliteConnection::connect("sqlite::memory:").await?;
    conn.query_file("sql_file_location").await?;
    Ok(())
}
