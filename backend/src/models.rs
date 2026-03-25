use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Person {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub object_id: Option<ObjectId>,
    pub id: i64,
    pub username: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Snipe {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub object_id: Option<ObjectId>,
    pub sniper_id: i64,
    pub snipee_id: i64,
    pub picture_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    pub channel_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guild_id: Option<i64>,
}
