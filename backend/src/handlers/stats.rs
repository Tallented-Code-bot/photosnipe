use mongodb::bson::doc;
use mongodb::Database;
use rocket::serde::json::Json;
use rocket::State;

use crate::models::{GlobalStats, LeaderboardEntry, Person};
use crate::responses::{ApiError, ApiResponse, PaginationParams};

/// GET /api/stats
/// Get global statistics
#[get("/stats")]
pub async fn get_global_stats(
    db: &State<Database>,
) -> Result<Json<ApiResponse<GlobalStats>>, ApiError> {
    let persons_collection = db.collection::<Person>("persons");
    let snipes_collection = db.collection::<mongodb::bson::Document>("snipes");

    // Count total persons
    let total_persons = persons_collection
        .count_documents(None, None)
        .await
        .map_err(|e| ApiError::internal_error(format!("Failed to count persons: {}", e)))?;

    // Count total snipes
    let total_snipes = snipes_collection
        .count_documents(None, None)
        .await
        .map_err(|e| ApiError::internal_error(format!("Failed to count snipes: {}", e)))?;

    // Get top sniper using aggregation
    let sniper_pipeline = vec![
        doc! {
            "$group": {
                "_id": "$sniper_id",
                "count": { "$sum": 1 }
            }
        },
        doc! {
            "$sort": { "count": -1 }
        },
        doc! {
            "$limit": 1
        },
    ];

    let mut sniper_cursor = snipes_collection
        .aggregate(sniper_pipeline, None)
        .await
        .map_err(|e| ApiError::internal_error(format!("Failed to get top sniper: {}", e)))?;

    let mut top_sniper = None;
    use futures::stream::StreamExt;
    if let Some(result) = sniper_cursor.next().await {
        let doc = result
            .map_err(|e| ApiError::internal_error(format!("Failed to read top sniper: {}", e)))?;
        if let (Some(sniper_id), Some(count)) = (doc.get_i64("_id").ok(), doc.get_i64("count").ok())
        {
            // Fetch the person
            if let Ok(Some(person)) = persons_collection
                .find_one(doc! { "id": sniper_id }, None)
                .await
            {
                top_sniper = Some(LeaderboardEntry { person, count });
            }
        }
    }

    // Get top snipee using aggregation
    let snipee_pipeline = vec![
        doc! {
            "$group": {
                "_id": "$snipee_id",
                "count": { "$sum": 1 }
            }
        },
        doc! {
            "$sort": { "count": -1 }
        },
        doc! {
            "$limit": 1
        },
    ];

    let mut snipee_cursor = snipes_collection
        .aggregate(snipee_pipeline, None)
        .await
        .map_err(|e| ApiError::internal_error(format!("Failed to get top snipee: {}", e)))?;

    let mut top_snipee = None;
    if let Some(result) = snipee_cursor.next().await {
        let doc = result
            .map_err(|e| ApiError::internal_error(format!("Failed to read top snipee: {}", e)))?;
        if let (Some(snipee_id), Some(count)) = (doc.get_i64("_id").ok(), doc.get_i64("count").ok())
        {
            // Fetch the person
            if let Ok(Some(person)) = persons_collection
                .find_one(doc! { "id": snipee_id }, None)
                .await
            {
                top_snipee = Some(LeaderboardEntry { person, count });
            }
        }
    }

    let stats = GlobalStats {
        total_persons: total_persons as i64,
        total_snipes: total_snipes as i64,
        top_sniper,
        top_snipee,
    };

    Ok(Json(ApiResponse::success(stats)))
}

/// GET /api/leaderboard/snipers?limit=10
/// Get top snipers (most snipes taken)
#[get("/leaderboard/snipers?<pagination..>")]
pub async fn get_top_snipers(
    db: &State<Database>,
    pagination: PaginationParams,
) -> Result<Json<ApiResponse<Vec<LeaderboardEntry>>>, ApiError> {
    pagination.validate()?;

    let persons_collection = db.collection::<Person>("persons");
    let snipes_collection = db.collection::<mongodb::bson::Document>("snipes");

    // Aggregate to get top snipers
    let pipeline = vec![
        doc! {
            "$group": {
                "_id": "$sniper_id",
                "count": { "$sum": 1 }
            }
        },
        doc! {
            "$sort": { "count": -1 }
        },
        doc! {
            "$skip": pagination.offset
        },
        doc! {
            "$limit": pagination.limit
        },
    ];

    let mut cursor = snipes_collection
        .aggregate(pipeline, None)
        .await
        .map_err(|e| ApiError::internal_error(format!("Failed to get leaderboard: {}", e)))?;

    let mut leaderboard = Vec::new();
    use futures::stream::StreamExt;
    while let Some(result) = cursor.next().await {
        let doc = result.map_err(|e| {
            ApiError::internal_error(format!("Failed to read leaderboard entry: {}", e))
        })?;

        if let (Some(sniper_id), Some(count)) = (doc.get_i64("_id").ok(), doc.get_i64("count").ok())
        {
            // Fetch the person
            if let Ok(Some(person)) = persons_collection
                .find_one(doc! { "id": sniper_id }, None)
                .await
            {
                leaderboard.push(LeaderboardEntry { person, count });
            }
        }
    }

    Ok(Json(ApiResponse::success(leaderboard)))
}

/// GET /api/leaderboard/snipees?limit=10
/// Get top snipees (most snipes received)
#[get("/leaderboard/snipees?<pagination..>")]
pub async fn get_top_snipees(
    db: &State<Database>,
    pagination: PaginationParams,
) -> Result<Json<ApiResponse<Vec<LeaderboardEntry>>>, ApiError> {
    pagination.validate()?;

    let persons_collection = db.collection::<Person>("persons");
    let snipes_collection = db.collection::<mongodb::bson::Document>("snipes");

    // Aggregate to get top snipees
    let pipeline = vec![
        doc! {
            "$group": {
                "_id": "$snipee_id",
                "count": { "$sum": 1 }
            }
        },
        doc! {
            "$sort": { "count": -1 }
        },
        doc! {
            "$skip": pagination.offset
        },
        doc! {
            "$limit": pagination.limit
        },
    ];

    let mut cursor = snipes_collection
        .aggregate(pipeline, None)
        .await
        .map_err(|e| ApiError::internal_error(format!("Failed to get leaderboard: {}", e)))?;

    let mut leaderboard = Vec::new();
    use futures::stream::StreamExt;
    while let Some(result) = cursor.next().await {
        let doc = result.map_err(|e| {
            ApiError::internal_error(format!("Failed to read leaderboard entry: {}", e))
        })?;

        if let (Some(snipee_id), Some(count)) = (doc.get_i64("_id").ok(), doc.get_i64("count").ok())
        {
            // Fetch the person
            if let Ok(Some(person)) = persons_collection
                .find_one(doc! { "id": snipee_id }, None)
                .await
            {
                leaderboard.push(LeaderboardEntry { person, count });
            }
        }
    }

    Ok(Json(ApiResponse::success(leaderboard)))
}

#[cfg(test)]
mod tests {
    fn check_mongodb_available() -> bool {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            match mongodb::Client::with_uri_str("mongodb://localhost:27017").await {
                Ok(client) => {
                    match client.database("admin").run_command(doc! { "ping": 1 }, None).await {
                        Ok(_) => true,
                        Err(_) => false,
                    }
                }
                Err(_) => false,
            }
        })
    }

    use super::*;
    use crate::models::{Person, Snipe};
    use mongodb::bson::doc;
    use rocket::local::blocking::Client;
    use rocket::http::Status;

    fn get_test_db() -> Database {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let client = mongodb::Client::with_uri_str("mongodb://localhost:27017")
                .await
                .expect("Failed to connect to MongoDB");
            client.database("photosnipe_test")
        })
    }

    fn setup_rocket(db: Database) -> rocket::Rocket<rocket::Build> {
        rocket::build()
            .manage(db)
            .mount(
                "/api",
                routes![
                    get_global_stats,
                    get_top_snipers,
                    get_top_snipees,
                ],
            )
    }

    fn setup_test_data(db: &Database) {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let persons_collection = db.collection::<Person>("persons");
            persons_collection.delete_many(
                doc! { "id": { "$in": [666666666, 555555555, 444444444] } }, 
                None
            ).await.ok();
            
            let snipes_collection = db.collection::<Snipe>("snipes");
            snipes_collection.delete_many(
                doc! { "sniper_id": { "$in": [666666666, 555555555, 444444444] } }, 
                None
            ).await.ok();

            persons_collection.insert_one(Person {
                object_id: None,
                id: 666666666,
                username: "top_sniper".to_string(),
                display_name: Some("Top Sniper".to_string()),
            }, None).await.ok();
            
            persons_collection.insert_one(Person {
                object_id: None,
                id: 555555555,
                username: "mid_sniper".to_string(),
                display_name: Some("Mid Sniper".to_string()),
            }, None).await.ok();
            
            persons_collection.insert_one(Person {
                object_id: None,
                id: 444444444,
                username: "low_sniper".to_string(),
                display_name: Some("Low Sniper".to_string()),
            }, None).await.ok();

            for _ in 0..5 {
                snipes_collection.insert_one(Snipe {
                    object_id: None,
                    sniper_id: 666666666,
                    snipee_id: 555555555,
                    picture_url: "https://test.com/image.jpg".to_string(),
                    text: Some("Test".to_string()),
                    channel_id: 123456,
                    guild_id: Some(789012),
                }, None).await.ok();
            }

            for _ in 0..3 {
                snipes_collection.insert_one(Snipe {
                    object_id: None,
                    sniper_id: 555555555,
                    snipee_id: 444444444,
                    picture_url: "https://test.com/image.jpg".to_string(),
                    text: Some("Test".to_string()),
                    channel_id: 123456,
                    guild_id: Some(789012),
                }, None).await.ok();
            }

            snipes_collection.insert_one(Snipe {
                object_id: None,
                sniper_id: 444444444,
                snipee_id: 666666666,
                picture_url: "https://test.com/image.jpg".to_string(),
                text: Some("Test".to_string()),
                channel_id: 123456,
                guild_id: Some(789012),
            }, None).await.ok();
        });
    }

    fn cleanup_test_data(db: &Database) {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let snipes_collection = db.collection::<Snipe>("snipes");
            snipes_collection.delete_many(
                doc! { "sniper_id": { "$in": [666666666, 555555555, 444444444] } }, 
                None
            ).await.ok();
            
            let persons_collection = db.collection::<Person>("persons");
            persons_collection.delete_many(
                doc! { "id": { "$in": [666666666, 555555555, 444444444] } }, 
                None
            ).await.ok();
        });
    }

    #[test]
    fn test_get_global_stats() {
        if !check_mongodb_available() {
            eprintln!("⚠️  Skipping test: MongoDB not available at localhost:27017");
            eprintln!("   Start MongoDB with: docker run -d -p 27017:27017 mongo");
            return;
        }
        let db = get_test_db();
        setup_test_data(&db);

        let client = Client::tracked(setup_rocket(db.clone())).expect("valid rocket instance");

        let response = client.get("/api/stats").dispatch();
        assert_eq!(response.status(), Status::Ok);
        let body = response.into_string().unwrap();
        assert!(body.contains("\"success\":true"));
        assert!(body.contains("total_persons"));
        assert!(body.contains("total_snipes"));

        cleanup_test_data(&db);
    }

    #[test]
    fn test_get_top_snipers() {
        if !check_mongodb_available() {
            eprintln!("⚠️  Skipping test: MongoDB not available at localhost:27017");
            eprintln!("   Start MongoDB with: docker run -d -p 27017:27017 mongo");
            return;
        }
        let db = get_test_db();
        setup_test_data(&db);

        let client = Client::tracked(setup_rocket(db.clone())).expect("valid rocket instance");

        let response = client.get("/api/leaderboard/snipers?limit=5").dispatch();
        assert_eq!(response.status(), Status::Ok);
        let body = response.into_string().unwrap();
        assert!(body.contains("\"success\":true"));

        cleanup_test_data(&db);
    }

    #[test]
    fn test_get_top_snipees() {
        if !check_mongodb_available() {
            eprintln!("⚠️  Skipping test: MongoDB not available at localhost:27017");
            eprintln!("   Start MongoDB with: docker run -d -p 27017:27017 mongo");
            return;
        }
        let db = get_test_db();
        setup_test_data(&db);

        let client = Client::tracked(setup_rocket(db.clone())).expect("valid rocket instance");

        let response = client.get("/api/leaderboard/snipees?limit=5").dispatch();
        assert_eq!(response.status(), Status::Ok);

        cleanup_test_data(&db);
    }
}
