use axum::{
    extract::{State},
    routing::{get, post, get_service},
    http::StatusCode,
    Json,
    Router, response::ErrorResponse,
};

//use tokio::stream;

use std::net::SocketAddr;

use tower_http::services::{ServeDir};

use serde::{Deserialize, Serialize};

use sqlx;

use chrono::{self, Datelike};


mod db;


#[derive(Clone)]
struct AppState {
    db: sqlx::Pool<sqlx::MySql> 
}


#[derive(Serialize, Deserialize, Debug)]
struct Row {

    id: i32,
    day: String,
    class: String,
    hour: String,
    fach_old: String,
    fach_new: String,
    away: String,
    sub: String,
    room: String,
    typ: String,
    info: String,


}

#[derive(Deserialize, Serialize, Debug, sqlx::FromRow, Clone)]
pub struct Date {

    pub datum: String,
    pub week_day: String,
}

#[derive(Deserialize, Debug)]
struct ID {
    id: i32,
}



#[tokio::main]
async fn main() {
    println!("Hello, world!");


    let state = AppState{db: db::connect().await};




    let app = Router::new()
    .nest_service("/", get_service(ServeDir::new("static/frontend"))
            .handle_error(|error: std::io::Error| async move {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Internal Error: {}", error)
            )
            })
    )
    .nest_service("/images", get_service(ServeDir::new("static/images"))
            .handle_error(|error: std::io::Error| async move {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Cant load images: {}", error)
            )
            }))
    .route("/add_row", post(add_row))
    .route("/get_rows", post(get_rows))
    .route("/get_day", post(get_day))
    .route("/add_day", post(add_day))
    .with_state(state);

    let addr = SocketAddr::from(([127,0,0,1], 7000));


    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
    .unwrap();

}




async fn add_row(State(state): State<AppState>, Json(payload): Json<Row>) -> StatusCode {

    println!("recieved row");

    println!("{:?}", payload);

    let day = Date {datum: "31.12.2022".to_owned(), week_day: "Monday".to_owned()};

    db::add_day(&state.db, &day).await;

    StatusCode::CREATED
}

async fn add_day(State(state): State<AppState>, Json(previous_day): Json<ID>) -> StatusCode{

    let date_str: String;

    match db::get_day(&state.db, previous_day.id).await {
        
        Ok(date) => {

            date_str = date.datum;     
        },
        Err(_e) => return StatusCode::INTERNAL_SERVER_ERROR,

    }


    let naive_date = chrono::NaiveDate::parse_from_str(&date_str, "%d.%m.%Y").unwrap();

    let mut next_day = naive_date.checked_add_days(chrono::Days::new(1)).unwrap();



    if next_day.weekday() == chrono::Weekday::Sat {

        next_day = next_day.checked_add_days(chrono::Days::new(2)).unwrap(); 
    }
    else if next_day.weekday() == chrono::Weekday::Sun {

        next_day = next_day.checked_add_days(chrono::Days::new(1)).unwrap(); 
    }

    let datum = next_day.format("%d.%m.%Y").to_string();

    let weekday = next_day.format("%A").to_string();

    let day = Date {
        datum: datum.to_owned(),
        week_day: weekday.to_owned(),
    };

    println!("add: {}, {}", day.datum, day.week_day);

    db::add_day(&state.db, &day).await;

    StatusCode::OK    
}

async fn get_day(State(state): State<AppState>, Json(day): Json<ID>) -> Result<Json<Date>, StatusCode>{

   
    println!("day was requested");

    match db::get_day(&state.db, day.id).await {
        
        Ok(date) => return Ok(Json(date)),
        Err(_e) => return Err(StatusCode::INTERNAL_SERVER_ERROR),

    }

}

async fn get_rows(Json(payload): Json<Date>) -> Result<Json<Row>, StatusCode> {


    if payload.datum == "17.01.2024" {



        let row = Row {
        
            id: 2,
            day: "17.01.2024".to_owned(),
            class: "11".to_owned(),
            hour: "1-2".to_owned(),
            fach_old: "F".to_owned(),
            fach_new: "-".to_owned(),
            away: "Malek".to_owned(),
            sub: "-".to_owned(),
            room: "-".to_owned(),
            typ: "EVA".to_owned(),
            info: "-".to_owned(),

        };
        
        return Ok(Json(row)) 
    }



    return Err(StatusCode::BAD_REQUEST);

}
