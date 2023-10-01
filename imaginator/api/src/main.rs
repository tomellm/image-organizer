#[allow(unused)]

mod routes;

use routes::*;

use image::codecs::jpeg::JpegDecoder;
use types::database::*; 
use axum::{
    routing::{get, post}, 
    Router, 
    extract::State, Json, 
};
use std::{env, fs, io::BufReader};
use sqlx::{mysql::MySqlPool,MySql, Pool};
use dotenv::dotenv;
use std::sync::Arc;
use image::io::Reader;

#[tokio::main]
async fn main() {
    dotenv().expect("Could not initialize dotenv crate");
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    let pool = MySqlPool
        ::connect(&env::var("DATABASE_URL").expect("Could not find DATABASE_URL env"))
        .await.expect("Failed to connect to DB");

    let app = Router::new()
        .route("/", get(get_all))
        .route("/insert-one", post(insert_one))
        .route("/clear", post(clear))
        .route("/images", get(read_images))
        .route("/image", get(read_image_stream))
        .route("/load", get(load))
        .with_state(Arc::new(pool));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}


