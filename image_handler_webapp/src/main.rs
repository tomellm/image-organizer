/*use rocket_contrib::templates::Template;
use std::collections::HashMap;
#[macro_use] extern crate rocket;

//localhost:8080/img_org
//
#[get("/")]
fn index() -> Template {
    let context = context();
    Template::render("index", &context)
}
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .attach(Template::fairing())
}*/

#[macro_use] extern crate rocket;

use rocket::Request;
use rocket::response::Redirect;

use rocket_dyn_templates::{Template, tera::Tera, context};

struct IndexResponse {
    num_files : String,
    all_files : Vec<String>,
    images_base_path: String
}

#[get("/")]
async fn index() -> Template {
    let context = context! {
        valid: "false"
        num_files: "",
        files: vec!["One", "Two", "Three"],
    };
    let response = reqwest::get("http://localhost:8080/img_org/").await;
    match response {
        Response => ,
        Error => 
    }
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
        .mount("/", routes![index, hello])
        .register("/", catchers![not_found])
        .attach(Template::fairing())
}