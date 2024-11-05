#[macro_use]
extern crate rocket;

use std::sync::Arc;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Request, Response};
use tokio::time::{interval, Duration};
use rocket::tokio::task;
use rocket::State;

mod config;
mod db;
mod models;
mod routes;

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "Content-Type"));
    }
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
async fn rocket() -> _ {
    let mongo_client = db::init_mongo().await;
    let database = Arc::new(mongo_client.database("search"));
    let collection = database.collection::<models::bing_image::BingImage>("bing_img");

    // 启动定时任务
    let collection_clone = collection.clone();
    task::spawn(async move {
        let mut interval = interval(Duration::from_secs(4 * 60 * 60));
        loop {
            interval.tick().await;
            let state = State::from(&collection_clone);
            routes::bing_image::get_today_bing_images(&state).await;
        }
    });

    rocket::build()
        .attach(CORS)
        .manage(collection)
        .mount("/", routes![index])
        .mount("/", routes::bing_image::all_routes())
}
