use sqlx::Row;

use dotenv::dotenv;

pub async fn db_test() -> Result<(), sqlx::Error> {

    dotenv().ok();

    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");


    let pool = sqlx::mysql::MySqlPool::connect(&url).await?;


    let res =  sqlx::query("SELECT 1 + 1 as sum")
        .fetch_one(&pool)
        .await?;

    let sum: i32 = res.get("sum");

    println!("result: {sum}");

    Ok(())
}

