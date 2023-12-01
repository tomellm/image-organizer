mod db;
#[allow(unused)]
mod routes;

use routes::*;

use axum::{
    routing::{get, post},
    Router,
};
use dotenv::dotenv;
use sqlx::mysql::MySqlPool;
use std::env;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    dotenv().expect("Could not initialize dotenv crate");
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    let pool =
        MySqlPool::connect(&env::var("DATABASE_URL").expect("Could not find DATABASE_URL env"))
            .await
            .expect("Failed to connect to DB");

    let app = Router::new()
        .route("/clear", post(clear))
        .route("/read-images", get(read_images))
        .route("/images", get(get_all))
        .route("/images", post(save_one))
        .with_state(Arc::new(pool));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
