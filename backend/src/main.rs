use axum::{
    routing::{get, post, get_service},
    http::StatusCode,
    Json,
    Router,
};

use tokio::stream;

use std::net::SocketAddr;

use tower_http::services::{ServeDir, ServeFile};


#[tokio::main]
async fn main() {
    println!("Hello, world!");


    let app = Router::new()
    .nest_service("/", get_service(ServeDir::new("static/frontend"))
            .handle_error(|error: std::io::Error| async move {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Internal Error: {}", error)
            )
            })
    );

    let addr = SocketAddr::from(([127,0,0,1], 7000));


    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
    .unwrap();
}
