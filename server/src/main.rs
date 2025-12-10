use dotenv::dotenv;
use std::env;
use axum::{
    routing::{get, post},
    Router,
};
use tower_http::cors::{Any, CorsLayer};

mod user;
mod auth;
mod image;
mod tags_requests;
mod ai;
mod user_groups;
mod common;


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
        .route("/api/images", get(crate::image::handler::handle_gallery))
        .route("/api/login", post(crate::auth::handler::login_user))
        .route("/api/image/{id}",get(crate::image::handler::handle_image))
        .route("/api/image/{id}/tags",post(crate::image::handler::handle_tag_post))
        .route("/api/tags/pending",get(crate::tags_requests::handler::handle_tags_requests_page))
        .route("/api/tags/{name}/approve",post(crate::tags_requests::handler::handle_tag_approval))
        .route("/api/tags/{name}/reject",post(crate::tags_requests::handler::handle_tag_rejection))
        .route("/api/image/{id}/ai",get(crate::ai::handler::handle_ai_suggestion_request))
        .route("/api/usergroups",get(crate::user_groups::handler::handle_user_groups_page))
        .route("/api/usergroups/adduser",post(crate::user_groups::handler::handle_user_groups_addition))
        .route("/api/usergroups/removeuser",post(crate::user_groups::handler::handle_user_groups_deletion))
        .layer(cors);

    let api_addr = env::var("API_ADDR").unwrap(); // TODO: replace unwrap

    let listener = tokio::net::TcpListener::bind(api_addr)
        .await
        .unwrap(); // TODO: replace unwrap
    axum::serve(listener, app).await.unwrap(); // TODO: replace unwrap
}
