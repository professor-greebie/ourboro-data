use sqlx::migrate::MigrateDatabase;
use std::env;
use sqlx::sqlite::SqlitePool;
use sqlx::Sqlite;
use crate::census::census_model::CensusLineStruct;

const DB_URL: &str = "sqlite://data/scratch/ourboro.db";

pub async fn create_db() -> Result<(), sqlx::Error> {
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| DB_URL.to_string());
    let pool = SqlitePool::connect(&database_url).await?;
    let conn = &pool;
    let sql = sqlx::query_file!("data/resources/quick/schema.sql");
    sql.execute(conn).await?;
    Ok(())
}

pub async fn add_census(year: i64) -> Result<(), sqlx::Error> {
    if !Sqlite::database_exists(&DB_URL).await? {
        create_db().await?;
    }
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| DB_URL.to_string());
    let pool = SqlitePool::connect(&database_url).await?;
    let conn = &pool;

    // Migration step to create the "census" tab

    let sql = sqlx::query!("INSERT INTO census (year) VALUES (?)", year);
    sql.execute(conn).await?;
    Ok(())
}

pub async fn add_count_result(census_line_struct: CensusLineStruct) -> Result<(), sqlx::Error> {
    if !Sqlite::database_exists(&DB_URL).await? {
        create_db().await?;
    }
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| DB_URL.to_string());
    let pool = SqlitePool::connect(&database_url).await?;
    let conn = &pool;
    let _characteristic = sqlx::query!(
        "INSERT OR IGNORE INTO characteristic (charId, charName, charNote) VALUES (?, ?, ?)",
        census_line_struct.characteristic_id,
        census_line_struct.characteristic_name,
        census_line_struct.characteristic_note
    );
    let _count_result = sqlx::query!(
        "INSERT OR IGNORE INTO count_result (dguid, charId, total_result, male_plus_result, female_plus_result, total_rate, male_plus_rate, female_plus_rate) VALUES (?, ?, ?, ?, ?, ?, ?, ?)", 
        census_line_struct.dguid, census_line_struct.characteristic_id, census_line_struct.c1_count_total, census_line_struct.c2_count_men_plus, census_line_struct.c3_count_women_plus, census_line_struct.c10_rate_total, census_line_struct.c11_rate_men_plus, census_line_struct.c12_rate_women_plus);
    let _location = sqlx::query!(
        "INSERT OR IGNORE INTO location (dguid, alt_geo_code, geo_level, geo_name) VALUES (?, ?, ?, ?)", 
        census_line_struct.dguid, census_line_struct.alt_geo_code, census_line_struct.geo_level, census_line_struct.geo_name);
    _characteristic.execute(conn).await?;
    _location.execute(conn).await?;
    _count_result.execute(conn).await?;

    //let sql = sqlx::query!("INSERT INTO count_result (census_id, postal_code, province, population, land_area, total_occupied_dwellings, total_single_detached_houses) VALUES (?, ?, ?, ?, ?, ?, ?)", 1, "K1A", "ON", 100, 100, 100, 100);
    //sql.execute(conn).await?;
    Ok(())
}
