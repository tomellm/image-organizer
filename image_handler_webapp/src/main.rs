use serde::Deserialize;

#[macro_use] extern crate rocket;

//localhost:8080/img_org
#[get("/")]
async fn index() -> &'static str {
    let data = reqwest::Client::new()
        .get("localhost:8080/img_org")
        .send()
        .await?
        .text()
        .await?;
    println!("{:#?}", data);
    return "Error";
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}