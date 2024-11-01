use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;

#[derive(Debug, Serialize, Deserialize)]
pub struct BingImage {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub url: String,
    pub title: String,
    pub copyright: String,
    pub startdate: String,
    pub enddate: String,
}