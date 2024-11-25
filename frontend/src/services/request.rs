use reqwest::Error;
use serde::Deserialize;

const BASE_URL: &str = "https://bing_images_api.virs.xyz";

#[derive(Deserialize, Debug, Clone)]
pub struct Image {
    pub url: String,
    pub title: String,
    pub copyright: String,
    pub copyrightlink: String,
    pub startdate: String,
    pub enddate: String,
    pub urlbase: String,
    pub fullstartdate: String,
    pub quiz: String,
    pub mkt: String,
    pub wp: bool,
    pub drk: i32,
    pub top: i32,
    pub bot: i32,
    pub hsh: String,
    pub hs: Vec<String>,
}

async fn get_request(endpoint: &str) -> Result<String, Error> {
    let url = format!("{}{}", BASE_URL, endpoint);
    let response = reqwest::get(&url).await?;
    let body = response.text().await?;
    Ok(body)
}

pub async fn get_images_list() -> Result<Vec<Image>, Box<dyn std::error::Error>> {
    let response = get_request("/bing_images").await?;
    let images: Vec<Image> = serde_json::from_str(&response)?;
    Ok(images)
}
