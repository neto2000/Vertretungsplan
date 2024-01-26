use axum::{
    routing::{get, post, get_service},
    http::StatusCode,
    Json,
    Router, response::ErrorResponse,
};

//use tokio::stream;

use std::net::SocketAddr;

use tower_http::services::{ServeDir};

use serde::{Deserialize, Serialize};


mod db;



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

#[derive(Deserialize, Debug)]
struct Date {

    date: String,
}



#[tokio::main]
async fn main() {
    println!("Hello, world!");

    match db::db_test().await {
        Ok(n) => n,
        Err(e) => println!("db error: {}", e),
    };


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
    .route("/get_rows", post(get_rows));

    let addr = SocketAddr::from(([127,0,0,1], 7000));


    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
    .unwrap();

}

async fn add_row(Json(payload): Json<Row>) -> StatusCode {

    println!("recieved row");

    println!("{:?}", payload);

    StatusCode::CREATED
}

async fn get_rows(Json(payload): Json<Date>) -> Result<Json<Row>, StatusCode> {


    if payload.date == "17.01.2024" {



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
