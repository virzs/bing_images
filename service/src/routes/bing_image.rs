use crate::models::bing_image::BingImage;
use mongodb::bson::{doc, Document};
use rocket::State;
use mongodb::{Collection};
use futures::stream::TryStreamExt;
use rocket::Route;
use rocket::serde::json::Json;
use rocket::serde::Deserialize;
use rocket::form::FromForm;
use rocket::yansi::Paint;
use rocket_okapi::{openapi, openapi_get_routes};
use schemars::JsonSchema;

#[derive(FromForm, Deserialize, JsonSchema)]
pub struct FilterParams {
    startdate: Option<String>,
    enddate: Option<String>,
    page: Option<i64>,
    page_size: Option<i64>,
}

async fn fetch_and_store_image(
    client: &reqwest::Client,
    url: &str,
    query_params: &[(&str, &str)],
    market: &str,
    base_url: &str,
    collection: &State<Collection<BingImage>>,
) {
    let res = client.get(url)
        .query(query_params)
        .send()
        .await
        .unwrap()
        .json::<serde_json::Value>()
        .await
        .unwrap();

    info!("调用预定任务Bing图像API成功. 区域 {}", market);

    if let Some(img) = res["images"].as_array().and_then(|arr| arr.get(0)) {
        let img: BingImage = serde_json::from_value(img.clone()).unwrap();

        info!("图像 {}", img.title.clone().unwrap_or("无标题".to_string()));

        let has = collection.find_one(doc! { "startdate": &img.startdate, "mkt": market }).await.unwrap();
        if has.is_none() {
            collection.insert_one(BingImage {
                url: format!("{}{}", base_url, img.url.unwrap_or_default()).into(),
                urlbase: format!("{}{}", base_url, img.urlbase.unwrap_or_default()).into(),
                quiz: format!("{}{}", base_url, img.quiz.unwrap_or_default()).into(),
                mkt: Some(market.to_string()),
                ..img
            }).await.unwrap();
            info!("{}", "添加图像成功".green());
        } else {
            info!("图像 {} 已存在", has.unwrap().title.clone().unwrap_or("无标题".to_string()));
        }
    }
}

pub async fn get_today_bing_images(collection: &State<Collection<BingImage>>) {
    let client = reqwest::Client::new();

    let locales = vec![
        ("https://cn.bing.com/HPImageArchive.aspx", "zh-CN", "https://cn.bing.com"),
        ("https://global.bing.com/HPImageArchive.aspx", "en-US", "https://www.bing.com"),
        // Add more locales here
    ];

    for (url, market, base_url) in locales {
        fetch_and_store_image(
            &client,
            url,
            &[("format", "js"), ("idx", "0"), ("n", "1"), ("mkt", market)],
            market,
            base_url,
            collection,
        ).await;
    }
}

fn build_filter(params: &FilterParams) -> Document {
    let mut filter = doc! {};

    if let Some(start) = &params.startdate {
        if let Some(end) = &params.enddate {
            filter.insert("enddate", doc! { "$gte": start, "$lte": end });
        } else {
            filter.insert("startdate", doc! { "$eq": start });
        }
    } else if let Some(end) = &params.enddate {
        filter.insert("enddate", doc! { "$eq": end });
    }

    filter
}

#[openapi]
#[get("/bing_images?<params..>")]
pub async fn get_bing_images(collection: &State<Collection<BingImage>>, params: FilterParams) -> Json<Vec<BingImage>> {
    let filter = build_filter(&params);

    let cursor = collection
        .inner()
        .find(filter)
        .sort(doc! { "enddate": -1 })
        .skip(((params.page.unwrap_or(1) - 1) * params.page_size.unwrap_or(10) as i64) as u64)
        .limit(params.page_size.unwrap_or(10) as i64)
        .await
        .unwrap();

    let images: Vec<BingImage> = cursor.try_collect().await.unwrap();

    Json(images)
}

pub fn all_routes() -> Vec<Route> {
    openapi_get_routes![get_bing_images]
}