use sqlx::{Row, Pool, MySql};

use dotenv::dotenv;

use crate::Date;

pub async fn connect() -> Pool<MySql> {

    dotenv().ok();

    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");


    let pool = sqlx::mysql::MySqlPool::connect(&url).await.expect("Failed to connect to DB");


    sqlx::migrate!("./migrations").run(&pool).await.expect("failed to migrate");


    let res =  sqlx::query("SELECT 1 + 1 as sum")
        .fetch_one(&pool)
        .await
        .expect("failed to fetch");

    let sum: i32 = res.get("sum");

    println!("result: {sum}");

    return pool;
}


pub async fn add_day(pool: &Pool<MySql>, day: &Date) {
    
    let query = "INSERT INTO day (datum, week_day) VALUES (?, ?)";

    sqlx::query(query)
        .bind(&day.date)
        .bind(&day.week_day)
        .execute(pool)
        .await
        .expect("day insertion failed");


}

pub async fn add_row(pool: &Pool<MySql>) {


}

