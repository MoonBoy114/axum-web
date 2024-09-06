use axum::{Router, routing::get, response::Json};
use serde::Serialize;
use std::net::SocketAddr;

#[derive(Serialize)]
struct Message {
    message: String,
}


async fn hello() -> Json<Message> {
    Json(Message {
        message: String::from("Hello, Axum"),
    })
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(hello));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);

    axum::Server::bind(&addr).serve(app.into_make_service())
    .await
    .unwrap();
}

