use crate::census::census_model::{CensusFilter, CensusLineStruct};
use crate::pccf::postal_code;
use postal_code::PostalCode;
use sqlx::migrate::MigrateDatabase;
use sqlx::sqlite::SqlitePool;
use sqlx::Sqlite;
use std::env;

const DB_URL: &str = "sqlite://data/scratch/ourboro.db";

enum KnownTable {
    AngusStart,
    AngusFinish,
    OISStart, 
    OISFinish,
    OurboroStart,
    OurboroFinish,
}

impl KnownTable {
    fn query(&self) -> &'static str {
        match self {
            KnownTable::AngusStart => "SELECT * FROM angus_data_a WHERE postal_code = ?",
            KnownTable::AngusFinish => "SELECT * FROM angus_data_b WHERE postal_code = ?",
            KnownTable::OISStart => "SELECT * FROM ois_data_a WHERE postal_code = ?",
            KnownTable::OISFinish => "SELECT * FROM ois_data_b WHERE postal_code = ?",
            KnownTable::OurboroStart => "SELECT * FROM ourboro_data_a WHERE postal_code = ?",
            KnownTable::OurboroFinish => "SELECT * FROM ourboro_data_b WHERE postal_code = ?",
        }
    }
}

pub fn generate_sql_query(
    census_items: Vec<CensusFilter>,
    postal_codes: Vec<String>,
) -> String {
    let census_query = census_items
        .iter()
        .map(|census_item| {
            format!(
                "MAX(CASE WHEN count_result.charId = {} THEN count_result.total_result END) AS {}\n",
                census_item.filter_column(),
                census_item.cache_name()
            )
        })
        .collect::<Vec<String>>()
        .join(", ");
    let postal_code_query = format!("WHERE postal_code_data.postal_code IN ({})", postal_codes.join(", "));
    format!("SELECT postal_code_data.postal_code, postal_code_data.lat, postal_code_data.lon, location.geo_name, \n{}
      FROM location
      INNER JOIN 
         postal_code_data ON location.alt_geo_code=SUBSTRING(postal_code_data.dguid, 1, LENGTH(postal_code_data.dguid))
      INNER JOIN
         count_result ON location.dguid=count_result.dguid
      {} 
      GROUP BY postal_code_data.postal_code", census_query, postal_code_query)
}
/**
pub async fn get_data_line_by_postal_codes_A(
    postal_codes: Vec<String>, table : KnownTable 
) -> Result<(), sqlx::Error> {
    for postal_code in postal_codes {
        let postal_code_query = sqlx::query!("SELECT * FROM angus_data_a WHERE postal_code=?", postal_code);
    }
    Ok(())
}
 
pub async fn get_data_line_by_postal_codes_B(
    postal_codes: Vec<String>, table : KnownTable 
) -> Result<(), sqlx::Error> {
    for postal_code in postal_codes {
        let postal_code_query = sqlx::query!("SELECT * FROM angus_data_b WHERE postal_code=?", postal_code);
    }
    Ok(())
}

*/

pub async fn add_pccf_entry(
    postal_code: PostalCode,
    sample: Option<bool>,
) -> Result<(), sqlx::Error> {
    if !Sqlite::database_exists(&DB_URL).await? {
        create_db().await?;
    }
    if sample.is_some() && sample.unwrap() {
        add_pccf_entry_sample(postal_code).await?;
    } else {
        add_pccf_entry_full(postal_code).await?;
    }
    Ok(())
}

pub async fn add_pccf_entry_full(postal_code: PostalCode) -> Result<(), sqlx::Error> {
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| DB_URL.to_string());
    let pool = SqlitePool::connect(&database_url).await?;
    let conn = &pool;
    let _pccf = sqlx::query!(
        "INSERT OR IGNORE INTO postal_code_data (postal_code, dguid, location_name, province, lat, lon, community_name) VALUES (?, ?, ?, ?, ?, ?, ?)",
        postal_code._postal_code, postal_code._dguid, postal_code._census_subdivision_name, postal_code._province, postal_code._point_latitude, postal_code._point_longitude, postal_code._community_name);
    _pccf.execute(conn).await?;
    Ok(())
}

pub async fn add_pccf_entry_sample(postal_code: PostalCode) -> Result<(), sqlx::Error> {
    let database_url = env::var("DATABASE_URL").unwrap_or_else(|_| DB_URL.to_string());
    let pool = SqlitePool::connect(&database_url).await?;
    let conn = &pool;
    let _pccf = sqlx::query!(
        "INSERT OR IGNORE INTO postal_code_data_sample (postal_code, dguid, location_name, province, lat, lon, community_name) VALUES (?, ?, ?, ?, ?, ?, ?)",
        postal_code._postal_code, postal_code._dguid, postal_code._census_subdivision_name, postal_code._province, postal_code._point_latitude, postal_code._point_longitude, postal_code._community_name);
    _pccf.execute(conn).await?;
    Ok(())
}

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
