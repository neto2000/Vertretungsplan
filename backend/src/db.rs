use sqlx::{mysql::MySqlQueryResult, MySql, Pool, Row};

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
        .bind(&day.datum)
        .bind(&day.week_day)
        .execute(pool)
        .await
        .expect("day insertion failed");


}

pub async fn add_row(pool: &Pool<MySql>, row: &crate::Row) {

    let query = "INSERT INTO plan (day, class, start_hour, end_hour, old_fach, new_fach, away, sub, room, typ, info) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)";

    sqlx::query(query)
        .bind(&row.day)
        .bind(&row.class)
        .bind(&row.start_hour)
        .bind(&row.end_hour)
        .bind(&row.old_fach)
        .bind(&row.new_fach)
        .bind(&row.away)
        .bind(&row.sub)
        .bind(&row.room)
        .bind(&row.typ)
        .bind(&row.info)
        .execute(pool)
        .await
        .expect("plan insertion failed");
}

pub async fn get_day(pool: &Pool<MySql>, id: i32) -> Result<Date, String>{


    let query = "SELECT datum, week_day FROM day WHERE id = ?";

    let days = sqlx::query_as::<_, Date>(query)
        .bind(id)
        .fetch_one(pool)
        .await;

    match days {
        Ok(date) => return Ok(date),
        Err(e) => {
            println!("{}",e);
            return Err("no exist".to_owned())
        },
    }

}

pub async fn get_rows(pool: &Pool<MySql>, day_id: i32) -> Result<Vec<crate::Row>, sqlx::Error> {

    let query = "SELECT * FROM plan WHERE day = ?";

    let rows = sqlx::query_as::<_, crate::Row>(query)
        .bind(day_id)
        .fetch_all(pool)
        .await;

    return rows;

}

pub async fn update_row(pool: &Pool<MySql>, new_row: crate::Row){

    let query = "UPDATE plan SET day = ?, class = ?, start_hour = ?, end_hour = ? , old_fach = ? , new_fach = ? , away = ? , sub = ?, room = ?, typ = ?, info = ? WHERE id = ?";


    sqlx::query(query)
        .bind(&new_row.day)
        .bind(&new_row.class)
        .bind(&new_row.start_hour)
        .bind(&new_row.end_hour)
        .bind(&new_row.old_fach)
        .bind(&new_row.new_fach)
        .bind(&new_row.away)
        .bind(&new_row.sub)
        .bind(&new_row.room)
        .bind(&new_row.typ)
        .bind(&new_row.info)
        .bind(&new_row.id)
        .execute(pool)
        .await
    .expect("cant update");
}

