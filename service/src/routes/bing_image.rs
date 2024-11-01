use crate::models::bing_image::BingImage;
use mongodb::bson::{doc, Document};
use rocket::State;
use mongodb::{Collection};
use futures::stream::TryStreamExt;
use rocket::Route;
use rocket::serde::json::Json;
use rocket::serde::Deserialize;
use rocket::form::FromForm;

#[derive(FromForm, Deserialize)]
pub struct FilterParams {
    startdate: Option<String>,
    enddate: Option<String>,
    // Add other filter parameters here
}

fn build_filter(params: &FilterParams) -> Document {
    let mut filter = doc! {};

    if let Some(start) = &params.startdate {
        filter.insert("startdate", doc! { "$gte": start });
    }

    if let Some(end) = &params.enddate {
        filter.insert("enddate", doc! { "$lte": end });
    }

    // Add other filter conditions here

    filter
}

#[get("/bing_images?<params..>")]
pub async fn get_bing_images(collection: &State<Collection<BingImage>>, params: FilterParams) -> Json<Vec<BingImage>> {
    let filter = build_filter(&params);
    let cursor = collection.find(filter).await.unwrap();
    let images: Vec<BingImage> = cursor.try_collect().await.unwrap();

    Json(images)
}

pub fn all_routes() -> Vec<Route> {
    routes![get_bing_images]
}