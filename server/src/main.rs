use std::net::SocketAddr;

use axum::{
    Router, 
    routing::{self},
    response::IntoResponse,
    http::StatusCode,
    Json
};
use tower_http::cors::{
    Any,
    CorsLayer
};

mod types;
use types::Person;

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new().allow_origin(Any);

    let app = Router::new()
        .route("/", routing::get(index))
        .route("/people", routing::get(get_people))
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap()
}

async fn index() -> &'static str {
    "Hi"
}

async fn get_people() -> impl IntoResponse {
    let people = vec![
        Person{
            name: String::from("Zachary"),
            age: 36,
            favourite_food: Some(String::from("steak"))
        },
        Person{
            name: String::from("Ebenezer"),
            age: 55,
            favourite_food: Some(String::from("pie"))
        },
        Person{
            name: String::from("Jordan"),
            age: 27,
            favourite_food: Some(String::from("coco pops"))
        },
    ];
    (StatusCode::OK, Json(people))
}
