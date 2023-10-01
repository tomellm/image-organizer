use image::codecs::jpeg::JpegDecoder;
use types::database::*; 
use axum::{
    routing::{get, post}, 
    Router, 
    extract::State, Json, 
};
use std::{env, fs, io::BufReader};
use sqlx::{mysql::MySqlPool,MySql, Pool, QueryBuilder};
use dotenv::dotenv;
use std::sync::Arc;
use image::io::Reader;

pub async fn read_images() -> Json<Vec<String>> {
    let arr: Vec<String>= fs::read_dir("../../working_files/original").unwrap().map(|d| {
        format!("{:?}", d.unwrap().path()).to_string()
    }).collect();
    Json(arr)
}


// camphoto_684387517 (5).jpg

pub async fn read_image_stream() -> Vec<u8> {
    fs::read("../../camphoto_684387517 (5).jpg").unwrap()
}

pub async fn clear(
    State(arc_pool): State<Arc<Pool<MySql>>>
) -> String {
    let result = sqlx::query!("truncate table images_data")
        .execute(&*arc_pool).await;

    match result {
        Ok(_) => "worked".to_string(),
        Err(_) => "didnt work".to_string()
    }
}


pub async fn get_all(
    State(arc_pool): State<Arc<Pool<MySql>>>
) -> Json<Vec<ImageData>> {
    let row:Vec<ImageData> = sqlx::query_as!(ImageData, "SELECT * FROM images_data")
        .fetch_all(&*arc_pool).await.expect("Could not execute fetch all for images_data");

   Json(row) 
}

pub async fn insert_one(
    State(arc_pool): State<Arc<Pool<MySql>>>,
    Json(input): Json<CreateImageData>
) -> String {
    let row = sqlx::query!("Insert into images_data (group_id, file_name, org_name, datetime, file_size) values ( 1, 'file', 'file', null, 100)")
        .execute(&*arc_pool).await.expect("could not fetch");

    format!("{:?}", row)
}


const BIND_LIMIT: usize = 65535;

pub async fn load(
    State(arc_pool): State<Arc<Pool<MySql>>>
) -> String {
    let env_var = env::var("ALLOWED_FORMATS").unwrap();
    let allowed_formats: Vec<&str>= env_var.split(' ').collect();

    let images = std::fs::read_dir(env::var("IMAGES_DIR").unwrap()).unwrap()
        .filter(|d| {
            if d.as_ref().is_err() {
                return false;
            }
            let binding = d.as_ref().unwrap().file_name();
            let file = binding.to_str().unwrap();
            allowed_formats.iter().any(|a| file.ends_with(a))
        }).map(|d| {
            let file = d.unwrap();

            let group_id = None;
            let file_name = file.file_name().into_string().unwrap();
            let org_name = file.file_name().into_string().unwrap();
            let datetime = None;
            let file_size = file.metadata().unwrap().len() as i32;

            CreateImageData {
                group_id,
                file_name,
                org_name,
                datetime,
                file_size,
            }
        }).collect::<Vec<CreateImageData>>();

    let mut query_builder: QueryBuilder<MySql> = QueryBuilder::new(
        // Note the trailing space; most calls to `QueryBuilder` don't automatically insert
        // spaces as that might interfere with identifiers or quoted strings where exact
        // values may matter.
        "INSERT INTO images_data(group_id, file_name, org_name, datetime, file_size) "
    );

    query_builder.push_values(images.clone().into_iter().take(BIND_LIMIT / 5), |mut b, image| {
        b
            .push_bind(image.group_id)
            .push_bind(image.file_name)
            .push_bind(image.org_name)
            .push_bind(image.datetime)
            .push_bind(image.file_size);
    });

    
    let mut query = query_builder.build();
    query.execute(&*arc_pool).await.expect("could not do insert");
    String::from("worked")
}
