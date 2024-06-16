use std::time::Duration;

use leptos::{create_resource, Resource, leptos_dom::logging, logging::log};
use leptos_query::{QueryResult, RefetchFn, QueryOptions, ResourceOption};
use types::image::*;

pub async fn get_images() -> Result<Vec<Media>, String> {
    let text_response = reqwest::get(
        format!("http://{}/images", dotenv!("API_URL"))
    )
        .await.map_err(|err| format!("{}", err))?
        .text()
        .await.map_err(|err| format!("{}", err))?;

    serde_json::from_str::<Result<Vec<Media>, ()>>(&text_response)
        .map_err(|err| format!("There was an error parsing the returned data: {}", err))?
        .map_err(|_| "There was an internal error fetching the images!".to_string())
}



pub async fn get_images_paginated(
    page: usize,
    per_page: usize
) -> Result<Option<Vec<Media>>, String> {
    let request = format!(
        "http://{}/images/paginated?page={}&per_page={}",
        dotenv!("API_URL"),
        page, per_page
    );
    log!("about to make the following request: {request}");
    let text_response = reqwest::get(request)
        .await.map_err(|err| format!("{}", err))?
        .text()
        .await.map_err(|err| format!("{}", err))?;

    serde_json::from_str::<Result<Option<Vec<Media>>, ()>>(&text_response)
        .map_err(|err| format!("There was an error parsing the returned data: {}", err))?
        .map_err(|_| "There was an internal error fetching the images!".to_string())
}
