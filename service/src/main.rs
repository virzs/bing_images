#[macro_use]
extern crate rocket;

use crate::routes::bing_image::all_routes;
use config::settings::get_settings_from_file;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::tokio::task;
use rocket::State;
use rocket::{Request, Response};
use rocket_okapi::{openapi, swagger_ui::*};
use std::sync::Arc;
use tokio::time::{interval, Duration};

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
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "GET, POST, PUT, DELETE",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "Content-Type"));
    }
}

#[openapi]
#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
async fn rocket() -> rocket::Rocket<rocket::Build> {
    let settings = get_settings_from_file("config.toml");
    let mongo_client = db::init_mongo(&settings).await;
    let database = Arc::new(mongo_client.database(&settings.mongo.database));
    let collection =
        database.collection::<models::bing_image::BingImage>(&settings.mongo.collection);

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
        .mount("/", all_routes())
        .mount(
            "/docs",
            make_swagger_ui(&SwaggerUIConfig {
                url: "/openapi.json".to_string(),
                ..Default::default()
            }),
        )
        .mount(
            "/openapi.json",
            rocket_okapi::openapi_get_routes![routes::bing_image::get_bing_images],
        )
}
