use dotenv::dotenv;
use std::env;
use axum::{
    routing::{get, post},
    Json, Router,
};
use serde::Serialize;
use tower_http::cors::{Any, CorsLayer};

mod user;
mod auth;
mod image;

#[derive(Serialize)] // convert struct to json string later
struct Image {
    id: u32,
    title: String,
    url: String,
}

#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() {
    dotenv().ok();

    // TODO
    // This is a very permissive setup for development
    let cors = CorsLayer::new()
        .allow_origin(Any) // Allows any origin
        .allow_methods(Any) // Allows any method (GET, POST, etc.)
        .allow_headers(Any); // Allows any header

    let app = Router::new()
        .route("/api/images", get(get_images))
        .route("/api/login", post(crate::auth::handler::login_user))
        .route("/api/image/{id}",get(crate::image::handler::handle_image))
        .route("/api/image/{id}/tags",post(crate::image::handler::handle_tag_post))
        .layer(cors);

    let api_addr = env::var("API_ADDR").unwrap(); // TODO: replace unwrap

    let listener = tokio::net::TcpListener::bind(api_addr)
        .await
        .unwrap(); // TODO: replace unwrap
    axum::serve(listener, app).await.unwrap(); // TODO: replace unwrap
}

async fn get_images() -> Json<Vec<Image>> {
    // TODO: currently, dummy data

    // TODO: not depend on frontend expected structure?
    let images = vec![
        Image {
            id: 1,
            title: "Ferris the Crab".to_string(),
            url: "https://rustacean.net/assets/cuddlyferris.png".to_string(),
        },
        Image {
            id: 2,
            title: "Google Logo".to_string(),
            url: "https://upload.wikimedia.org/wikipedia/commons/thumb/7/77/Google_Images_2015_logo.svg/1200px-Google_Images_2015_logo.svg.png".to_string(),
        },
    ];

    Json(images)
}
