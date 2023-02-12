#[macro_use] extern crate rocket;

use std::error::Error;

use rocket::Request;
use rocket::response::Redirect;

use rocket_dyn_templates::{Template, tera::Tera, context};
use serde::Deserialize;

#[derive(Deserialize)]
struct IndexResponse {
    num_files : i32,
    all_files : Vec<String>,
    images_base_path: String
}

#[get("/")]
async fn index() -> Template {
    let mut response_valid: bool = false; 
    let mut json_response: IndexResponse = IndexResponse{
        num_files: -1,
        all_files: vec!["null".to_string()],
        images_base_path: "null".to_string()
    };
    let response = reqwest::get("http://localhost:8080/img_org/")
                                        .await
                                        .expect("The Json fetch did not work");
    match response.json().await {
        Ok(res) => {
            response_valid = true;
            json_response = res;
        },
        Err(err) => {
            info!("{}", err.to_string());
        }
    }

    let context = context! {
        valid: response_valid.to_string(),
        num_files: json_response.num_files,
        all_files: json_response.all_files,
        images_base_path: json_response.images_base_path
    };

    
    Template::render("index", context)
}

#[catch(404)]
fn not_found(req: &Request<'_>) -> Template {
    Template::render("/error/404", context! {
        uri: req.uri()
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .register("/", catchers![not_found])
        .attach(Template::fairing())
}


