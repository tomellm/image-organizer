#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;

use std::sync::Mutex;
use std::collections::HashMap;

use rocket::serde::json::Json;

// The type to represent the ID of a message.
type ID = usize;

// We're going to store all of the messages here. No need for a DB.
type MessageMap = Mutex<HashMap<ID, String>>;

#[derive(Deserialize)]
struct IndexResponse {
    num_files : i32,
    all_files : Vec<String>,
    images_base_path: String
}

#[get("/")]
async fn index() -> Json<&'static str> {
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
    
    /*return Some(json!({
        "valid": response_valid.to_string(),
        "num_files": json_response.num_files,
        "all_files": json_response.all_files,
        "images_base_path": json_response.images_base_path
    }));*/

    Json("{
        'valid': 'response_valid.to_string()',
        'num_files': 'json_response.num_files',
        'all_files': 'json_response.all_files',
        'images_base_path': 'json_response.images_base_path'
    }")

}

#[get("/ftp")]
async fn ftp() -> Json<&'static str> {
    
}

#[catch(404)]
fn not_found() -> Json<&'static str> {
    Json("{
        'status': 'error',
        'reason': 'Resource was not found'
    }")
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, ftp])
        .register("/", catchers![not_found])
}


