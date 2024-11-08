use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;
use schemars::JsonSchema;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct BingImage {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    #[schemars(skip)]
    pub id: Option<ObjectId>,
    pub url: Option<String>,
    pub title: Option<String>,
    pub copyright: Option<String>,
    pub copyrightlink: Option<String>,
    pub startdate: Option<String>,
    pub enddate: Option<String>,
    pub urlbase: Option<String>,
    pub fullstartdate: Option<String>,
    pub quiz: Option<String>,
    pub mkt: Option<String>,
    pub wp: Option<bool>,
    pub drk: Option<i64>,
    pub top: Option<i64>,
    pub bot: Option<i64>,
    pub hsh: Option<String>,
    pub hs: Option<Vec<serde_json::Value>>,
}