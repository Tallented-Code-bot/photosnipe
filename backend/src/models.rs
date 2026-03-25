use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Person {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub object_id: Option<ObjectId>,
    pub id: i64,
    pub username: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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

/// Input DTO for creating a new person
#[derive(Debug, Deserialize)]
pub struct CreatePersonInput {
    pub id: i64,
    pub username: String,
    pub display_name: Option<String>,
}

/// Input DTO for updating a person
#[derive(Debug, Deserialize)]
pub struct UpdatePersonInput {
    pub username: Option<String>,
    pub display_name: Option<String>,
}

/// Input DTO for creating a new snipe
#[derive(Debug, Deserialize)]
pub struct CreateSnipeInput {
    pub sniper_id: i64,
    pub snipee_id: i64,
    pub picture_url: String,
    pub text: Option<String>,
    pub channel_id: i64,
    pub guild_id: Option<i64>,
}

/// Input DTO for updating a snipe
#[derive(Debug, Deserialize)]
pub struct UpdateSnipeInput {
    pub picture_url: Option<String>,
    pub text: Option<String>,
}

/// Statistics for a person
#[derive(Debug, Serialize)]
pub struct PersonStats {
    pub person: Person,
    pub snipes_taken: i64,
    pub snipes_received: i64,
}

/// Global statistics
#[derive(Debug, Serialize)]
pub struct GlobalStats {
    pub total_persons: i64,
    pub total_snipes: i64,
    pub top_sniper: Option<LeaderboardEntry>,
    pub top_snipee: Option<LeaderboardEntry>,
}

/// Leaderboard entry
#[derive(Debug, Serialize)]
pub struct LeaderboardEntry {
    pub person: Person,
    pub count: i64,
}
