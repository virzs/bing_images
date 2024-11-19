use crate::models::bing_image::BingImage;
use futures::stream::TryStreamExt;
use mongodb::bson::{doc, Document};
use mongodb::Collection;
use rocket::form::FromForm;
use rocket::serde::json::Json;
use rocket::serde::Deserialize;
use rocket::yansi::Paint;
use rocket::Route;
use rocket::State;
use rocket_okapi::{openapi, openapi_get_routes};
use schemars::JsonSchema;

#[derive(FromForm, Deserialize, JsonSchema)]
pub struct FilterParams {
    startdate: Option<String>,
    enddate: Option<String>,
    page: Option<i64>,
    page_size: Option<i64>,
}

fn create_bing_image(img: BingImage, base_url: &str, market: &str) -> BingImage {
    BingImage {
        url: format!("{}{}", base_url, img.url.as_deref().unwrap_or_default()).into(),
        urlbase: format!("{}{}", base_url, img.urlbase.as_deref().unwrap_or_default()).into(),
        quiz: format!("{}{}", base_url, img.quiz.as_deref().unwrap_or_default()).into(),
        mkt: Some(market.to_string()),
        ..img
    }
}

async fn fetch_and_store_image(
    client: &reqwest::Client,
    url: &str,
    query_params: &[(&str, &str)],
    market: &str,
    base_url: &str,
    collection: &State<Collection<BingImage>>,
) {
    let res = client
        .get(url)
        .query(query_params)
        .send()
        .await
        .unwrap()
        .json::<serde_json::Value>()
        .await
        .unwrap();

    info!("调用预定任务Bing图像API成功. 区域 {}", market);

    if let Some(images) = res["images"].as_array() {
        for img_value in images {
            let img: BingImage = serde_json::from_value(img_value.clone()).unwrap();

            info!("图像 {}", img.title.clone().unwrap_or("无标题".to_string()));

            let has = collection
                .find_one(doc! { "startdate": &img.startdate, "mkt": market })
                .await
                .unwrap();
            if has.is_none() {
                collection
                    .insert_one(create_bing_image(img, base_url, market))
                    .await
                    .unwrap();
                info!("{}", "添加图像成功".green());
            } else {
                // 如果已存在的数据title和新数据的title不一致，替换数据
                let local_img = has.unwrap();

                if local_img.title != img.title {
                    info!("本地图像标题: {}", local_img.title.clone().unwrap_or("无标题".to_string()));
                    info!("新图像标题: {}", img.title.clone().unwrap_or("无标题".to_string()));
                    collection
                        .replace_one(
                            doc! { "startdate": &img.startdate, "mkt": market },
                            create_bing_image(img, base_url, market),
                        )
                        .await
                        .unwrap();
                    info!("{}", "替换图像成功".green());
                } else {
                    info!("{}", "图像已存在".yellow());
                }
            }
        }
    }
}

pub async fn get_today_bing_images(collection: &State<Collection<BingImage>>) {
    let client = reqwest::Client::new();

    let locales = vec![
        (
            "https://cn.bing.com/HPImageArchive.aspx",
            "zh-CN",
            "https://cn.bing.com",
        ),
        (
            "https://global.bing.com/HPImageArchive.aspx",
            "en-US",
            "https://www.bing.com",
        ),
        // Add more locales here
    ];

    for (url, market, base_url) in locales {
        fetch_and_store_image(
            &client,
            url,
            &[("format", "js"), ("idx", "0"), ("n", "8"), ("mkt", market)],
            market,
            base_url,
            collection,
        )
            .await;
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
pub async fn get_bing_images(
    collection: &State<Collection<BingImage>>,
    params: FilterParams,
) -> Json<Vec<BingImage>> {
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
