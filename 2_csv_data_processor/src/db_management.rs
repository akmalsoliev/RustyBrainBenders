extern crate sqlx;

use sqlx::Row;
use std::error::Error;

pub async fn sql() -> Result<(), Box<dyn Error>> {
    let url = "postgres://admin:admin@localhost:5432";
    let pool = sqlx::postgres::PgPool::connect(url)
        .await
        .expect("Failed to connect");

    let res = sqlx::query("SELECT 1 + 1 AS SUM_")
        .fetch_one(&pool)
        .await
        .expect("Failed to query");

    let get_sum: i32 = res.get("sum");

    println!("1 + 1 = {get_sum}");

    Ok(())
}
