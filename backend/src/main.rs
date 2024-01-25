use axum::{
    routing::{get, post, get_service},
    http::StatusCode,
    Json,
    Router,
};

use tokio::stream;

use std::net::SocketAddr;

use tower_http::services::{ServeDir, ServeFile};


mod db;


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
            }));

    let addr = SocketAddr::from(([127,0,0,1], 7000));


    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
    .unwrap();
}
