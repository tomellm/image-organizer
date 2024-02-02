use cfg_if::cfg_if;
use leptos::{server, ServerFnError};

cfg_if! { if #[cfg(feature = "ssr")] {

use crate::{db, state};
use chrono::NaiveDateTime;
use cloud_storage::{Client, client::ObjectClient};
use dateparser::DateTimeUtc;
use image::{io::Reader, codecs::jpeg::JpegEncoder};
use indradb::{BulkInsertItem, QueryExt};
use rayon::prelude::*;
use axum::{
    extract::{State, Path, Query},
    Json,
};
use sqlx::{MySql, Pool};
use futures::future;
use tracing::{event, Level};
use uuid::Uuid;
use std::{sync::Arc, fs::DirEntry, collections::HashMap, io::Cursor, ops::Deref};
use std::fs;
use types::{args::{ImageCreateArgs, Pagination, GetManyPayload}, image::Media};

#[server(
    prefix = "/images",
    endpoint = "read"
)]
pub async fn read_images() -> Result<Vec<String>, ServerFnError> {
    let arr: Vec<String> = fs::read_dir(std::env!("IMAGES_DIR"))
        .unwrap()
        .map(|d| format!("{:?}", d.unwrap().path()).to_string())
        .collect();
    Ok(arr)
}

pub async fn get_one(
    State(database): State<state::Database>,
    Path(uuid): Path<Uuid>
) -> Json<Result<Media, ()>> {
    let media = db::get_media(database.0, uuid).await;
    Json(media)
}

pub async fn get_many(
    State(database): State<state::Database>,
    Json(uuids): Json<GetManyPayload>
) -> Json<Result<Vec<Media>, ()>> {
    Json(db::get_many_media(database.0, uuids.uuids).await)
}

pub async fn get_all(
   State(database): State<state::Database> 
) -> Json<Vec<Media>> {
    let images = db::get_all_medias(database.0).await.unwrap();
    Json(images)
}

pub async fn read_and_save_images(
   State(database): State<state::Database> 
) -> Json<(Vec<Media>, Vec<String>)> {
    let (successes, errors): (Vec<_>, Vec<_>) = fs::read_dir(std::env!("IMAGES_DIR"))
         .unwrap().into_iter().partition(Result::is_ok);

    let file_map = successes.into_iter()
        .map(Result::unwrap)
        .fold(
            HashMap::new() as HashMap<String, (Option<DirEntry>, Option<DirEntry>)>,
            |mut map, entry| {
                let stem = entry.path().file_stem().unwrap().to_str().unwrap().to_string();
                let path = entry.path();
                let extension = path.extension().unwrap().to_str().unwrap();
                let element = match map.remove(&stem) {
                    Some(element) => element,
                    None => (None, None)
                };

                match extension {
                    "xmp" => { map.insert(stem, (element.0, Some(entry)));},
                    _ => { map.insert(stem, (Some(entry), element.1));}
                };
                map
            }
        );

    let images = file_map.into_iter()
        .filter(|(_, val)| {
            match val {
                (Some(_), _) => true,
                _ => false
            }
        })
        .map(|(_, (entry, xmp_file))| {
            Media::from_dir_entry(entry.unwrap(), xmp_file)
        })
        .collect::<Vec<_>>();

    let errors: Vec<String> = errors.into_iter()
        .map(Result::unwrap_err)
        .map(|e| format!("{e}"))
        .collect();

    let _ = db::save_medias(database.0.clone(), images.clone()).await;

    Json((images, errors))
}

// camphoto_684387517 (5).jpg

pub async fn read_image_stream() -> Vec<u8> {
    fs::read("../../camphoto_684387517 (5).jpg").unwrap()
}

pub async fn clear(
    State(database): State<state::Database> 
) -> Result<(), ()> {
    let _ = sqlx::query!("truncate table media_data")
        .execute(&*database.0)
        .await 
        .or(Err(()))?;

    let _ = sqlx::query!("truncate table metadata")
        .execute(&*database.0)
        .await 
        .or(Err(()))?;

    let _ = sqlx::query!("truncate table xmpdata")
        .execute(&*database.0)
        .await 
        .or(Err(()))?;

    Ok(())

}

pub async fn save_one(
    State(database): State<state::Database>,
    Json(input): Json<ImageCreateArgs>,
    ) -> Json<Media> {
    let image = Media::from_args(input);
    let _ = db::save_media(database.0, image.clone()).await;

    Json(image)
}

pub async fn get_all_images(
    State(database): State<state::Database>
) -> Json<Result<Vec<Media>, ()>> {
    Json(db::get_all_images(database.0).await)
}




pub async fn get_images_paginated(
    State(database): State<state::Database>,
    pagination: Query<Pagination>
) -> Json<Result<Option<Vec<Media>>, ()>> {
    let pagination = pagination.0;
    event!(Level::DEBUG, "pagination {pagination:?}");
    let media = db::get_images_paginated(database.0, pagination).await;
    event!(Level::DEBUG, "number of images: {:?}", media.clone().unwrap().unwrap().len());
    Json(media)
}


pub async fn count_query(
    State(graph_db): State<state::GraphDB>,
) -> Json<String> {
    let mut graph_db = graph_db.0.lock().await;
    let all_vert = graph_db.get(
        indradb::CountQuery::new(Box::new(indradb::Query::AllVertex)).unwrap()
    ).await.unwrap();
    let all_edge = graph_db.get(
        indradb::CountQuery::new(Box::new(indradb::Query::AllEdge)).unwrap()
    ).await.unwrap();


    Json(format!("verts: {:?}, edges: {:?}", all_vert, all_edge))
}

pub async fn create_graph(
    State(state): State<state::ApiState>
) -> Json<String> {
    let pool = state.databse.0;
    let mut graph_db = state.graph_db.0.lock().await;

    let medias = db::get_all_medias(pool.clone()).await.unwrap();
    let date_identifier = indradb::Identifier::new("date-created").unwrap();

    let mut items = medias.into_iter()
        .map(|m| {
            let vert = indradb::Vertex::with_id(
                m.uuid, indradb::Identifier::new("Media").unwrap()
            );

            let date = m.datetime_created.map(|date| {
                BulkInsertItem::VertexProperty(
                    m.uuid,
                    date_identifier,
                    indradb::Json::new(
                        serde_json::to_value(&date).unwrap()
                    )
                )
            });

            (
                BulkInsertItem::Vertex(vert),
                date
            )
        })
        .fold(vec![], |mut acc, (vert, o_date)| {
            acc.push(vert);
            if let Some(date) = o_date {
                acc.push(date);
            }

            acc
        });

    println!("len items: {}", items.len());

    graph_db.bulk_insert(items).await.unwrap();
    graph_db.sync().await.unwrap();



    let all_vert = graph_db.get(indradb::Query::AllVertex).await.unwrap();
    let all_edge = graph_db.get(indradb::Query::AllEdge).await.unwrap();
    
    Json(format!("verts: {:?}, edges: {:?}", all_vert, all_edge))
}

pub async fn properties(
    State(graph_db): State<state::GraphDB>
) -> Json<String> {
    let mut graph_db = graph_db.0.lock().await;

    /*let all_prop = graph_db.get(
        indradb::PipePropertyQuery::new(
            Box::new(indradb::Query::AllVertex)
        ).unwrap()
    ).await.unwrap();*/

    let all_prop = graph_db.get(
        indradb::AllVertexQuery.properties().unwrap()
    ).await.unwrap();

    Json(format!("{:?}", all_prop))
}

pub async fn delete_all(
    State(graph_db): State<state::GraphDB>
) -> Json<String> {
    let mut graph_db = graph_db.0.lock().await;
    graph_db.delete(indradb::Query::AllEdge).await.unwrap();
    graph_db.delete(indradb::Query::AllVertex).await.unwrap();

    graph_db.sync().await.unwrap();

    let all_vert = graph_db.get(indradb::Query::AllVertex).await.unwrap();
    let all_edge = graph_db.get(indradb::Query::AllEdge).await.unwrap();
    
    Json(format!("verts: {:?}, edges: {:?}", all_vert, all_edge))
}
}}
