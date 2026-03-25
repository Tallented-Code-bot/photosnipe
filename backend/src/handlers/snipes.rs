use mongodb::bson::doc;
use mongodb::bson::oid::ObjectId;
use mongodb::Database;
use rocket::serde::json::Json;
use rocket::State;

use crate::models::{CreateSnipeInput, Person, Snipe, UpdateSnipeInput};
use crate::responses::{ApiError, ApiResponse, PaginationParams, ResponseMeta, SnipeSearchParams};

/// GET /api/snipes?limit=50&offset=0
/// List all snipes with pagination
#[get("/snipes?<pagination..>")]
pub async fn list_snipes(
    db: &State<Database>,
    pagination: PaginationParams,
) -> Result<Json<ApiResponse<Vec<Snipe>>>, ApiError> {
    pagination.validate()?;

    let collection = db.collection::<Snipe>("snipes");

    // Get total count
    let total = collection
        .count_documents(None, None)
        .await
        .map_err(|e| ApiError::internal_error(format!("Failed to count snipes: {}", e)))?;

    // Get paginated results
    let options = mongodb::options::FindOptions::builder()
        .skip(Some(pagination.offset as u64))
        .limit(Some(pagination.limit))
        .build();

    let mut cursor = collection
        .find(None, options)
        .await
        .map_err(|e| ApiError::internal_error(format!("Failed to fetch snipes: {}", e)))?;

    let mut snipes = Vec::new();
    use futures::stream::StreamExt;
    while let Some(result) = cursor.next().await {
        match result {
            Ok(snipe) => snipes.push(snipe),
            Err(e) => {
                return Err(ApiError::internal_error(format!("Failed to read snipe: {}", e)))
            }
        }
    }

    let meta = ResponseMeta::new(total as i64, pagination.limit, pagination.offset);
    Ok(Json(ApiResponse::with_meta(snipes, meta)))
}

/// GET /api/snipes/:object_id
/// Get a specific snipe by MongoDB ObjectId
#[get("/snipes/<object_id>")]
pub async fn get_snipe(
    db: &State<Database>,
    object_id: String,
) -> Result<Json<ApiResponse<Snipe>>, ApiError> {
    let oid = ObjectId::parse_str(&object_id)
        .map_err(|_| ApiError::bad_request("Invalid ObjectId format".to_string()))?;

    let collection = db.collection::<Snipe>("snipes");

    let snipe = collection
        .find_one(doc! { "_id": oid }, None)
        .await
        .map_err(|e| ApiError::internal_error(format!("Database error: {}", e)))?
        .ok_or_else(|| ApiError::not_found(format!("Snipe with id {} not found", object_id)))?;

    Ok(Json(ApiResponse::success(snipe)))
}

/// GET /api/snipes/by-channel/:channel_id?limit=50&offset=0
/// Get snipes from a specific channel
#[get("/snipes/by-channel/<channel_id>?<pagination..>")]
pub async fn get_snipes_by_channel(
    db: &State<Database>,
    channel_id: i64,
    pagination: PaginationParams,
) -> Result<Json<ApiResponse<Vec<Snipe>>>, ApiError> {
    pagination.validate()?;

    let collection = db.collection::<Snipe>("snipes");

    // Get total count
    let filter = doc! { "channel_id": channel_id };
    let total = collection
        .count_documents(filter.clone(), None)
        .await
        .map_err(|e| ApiError::internal_error(format!("Failed to count snipes: {}", e)))?;

    // Get paginated results
    let options = mongodb::options::FindOptions::builder()
        .skip(Some(pagination.offset as u64))
        .limit(Some(pagination.limit))
        .build();

    let mut cursor = collection
        .find(filter, options)
        .await
        .map_err(|e| ApiError::internal_error(format!("Failed to fetch snipes: {}", e)))?;

    let mut snipes = Vec::new();
    use futures::stream::StreamExt;
    while let Some(result) = cursor.next().await {
        match result {
            Ok(snipe) => snipes.push(snipe),
            Err(e) => {
                return Err(ApiError::internal_error(format!("Failed to read snipe: {}", e)))
            }
        }
    }

    let meta = ResponseMeta::new(total as i64, pagination.limit, pagination.offset);
    Ok(Json(ApiResponse::with_meta(snipes, meta)))
}

/// GET /api/snipes/by-guild/:guild_id?limit=50&offset=0
/// Get snipes from a specific guild
#[get("/snipes/by-guild/<guild_id>?<pagination..>")]
pub async fn get_snipes_by_guild(
    db: &State<Database>,
    guild_id: i64,
    pagination: PaginationParams,
) -> Result<Json<ApiResponse<Vec<Snipe>>>, ApiError> {
    pagination.validate()?;

    let collection = db.collection::<Snipe>("snipes");

    // Get total count
    let filter = doc! { "guild_id": guild_id };
    let total = collection
        .count_documents(filter.clone(), None)
        .await
        .map_err(|e| ApiError::internal_error(format!("Failed to count snipes: {}", e)))?;

    // Get paginated results
    let options = mongodb::options::FindOptions::builder()
        .skip(Some(pagination.offset as u64))
        .limit(Some(pagination.limit))
        .build();

    let mut cursor = collection
        .find(filter, options)
        .await
        .map_err(|e| ApiError::internal_error(format!("Failed to fetch snipes: {}", e)))?;

    let mut snipes = Vec::new();
    use futures::stream::StreamExt;
    while let Some(result) = cursor.next().await {
        match result {
            Ok(snipe) => snipes.push(snipe),
            Err(e) => {
                return Err(ApiError::internal_error(format!("Failed to read snipe: {}", e)))
            }
        }
    }

    let meta = ResponseMeta::new(total as i64, pagination.limit, pagination.offset);
    Ok(Json(ApiResponse::with_meta(snipes, meta)))
}

/// GET /api/snipes/search?sniper_id=&snipee_id=&channel_id=&guild_id=&has_text=&limit=50&offset=0
/// Search snipes with various filters
#[get("/snipes/search?<params..>")]
pub async fn search_snipes(
    db: &State<Database>,
    params: SnipeSearchParams,
) -> Result<Json<ApiResponse<Vec<Snipe>>>, ApiError> {
    let pagination = params.pagination();
    pagination.validate()?;

    let collection = db.collection::<Snipe>("snipes");

    // Build filter document
    let mut filter = mongodb::bson::Document::new();

    if let Some(sniper_id) = params.sniper_id {
        filter.insert("sniper_id", sniper_id);
    }
    if let Some(snipee_id) = params.snipee_id {
        filter.insert("snipee_id", snipee_id);
    }
    if let Some(channel_id) = params.channel_id {
        filter.insert("channel_id", channel_id);
    }
    if let Some(guild_id) = params.guild_id {
        filter.insert("guild_id", guild_id);
    }
    if let Some(has_text) = params.has_text {
        if has_text {
            filter.insert("text", doc! { "$exists": true, "$ne": null });
        } else {
            filter.insert(
                "text",
                doc! { "$or": [{ "$exists": false }, { "$eq": null }] },
            );
        }
    }

    // Get total count with filters
    let total = collection
        .count_documents(filter.clone(), None)
        .await
        .map_err(|e| ApiError::internal_error(format!("Failed to count snipes: {}", e)))?;

    // Get paginated results
    let options = mongodb::options::FindOptions::builder()
        .skip(Some(pagination.offset as u64))
        .limit(Some(pagination.limit))
        .build();

    let mut cursor = collection
        .find(filter, options)
        .await
        .map_err(|e| ApiError::internal_error(format!("Failed to search snipes: {}", e)))?;

    let mut snipes = Vec::new();
    use futures::stream::StreamExt;
    while let Some(result) = cursor.next().await {
        match result {
            Ok(snipe) => snipes.push(snipe),
            Err(e) => {
                return Err(ApiError::internal_error(format!("Failed to read snipe: {}", e)))
            }
        }
    }

    let meta = ResponseMeta::new(total as i64, pagination.limit, pagination.offset);
    Ok(Json(ApiResponse::with_meta(snipes, meta)))
}

/// POST /api/snipes
/// Create a new snipe
#[post("/snipes", data = "<input>")]
pub async fn create_snipe(
    db: &State<Database>,
    input: Json<CreateSnipeInput>,
) -> Result<Json<ApiResponse<Snipe>>, ApiError> {
    // Validate input
    if input.picture_url.is_empty() {
        return Err(ApiError::validation_error(
            "Picture URL cannot be empty".to_string(),
        ));
    }

    // Check that sniper exists
    let persons_collection = db.collection::<Person>("persons");
    persons_collection
        .find_one(doc! { "id": input.sniper_id }, None)
        .await
        .map_err(|e| ApiError::internal_error(format!("Database error: {}", e)))?
        .ok_or_else(|| {
            ApiError::validation_error(format!("Sniper with id {} not found", input.sniper_id))
        })?;

    // Check that snipee exists
    persons_collection
        .find_one(doc! { "id": input.snipee_id }, None)
        .await
        .map_err(|e| ApiError::internal_error(format!("Database error: {}", e)))?
        .ok_or_else(|| {
            ApiError::validation_error(format!("Snipee with id {} not found", input.snipee_id))
        })?;

    let snipe = Snipe {
        object_id: None,
        sniper_id: input.sniper_id,
        snipee_id: input.snipee_id,
        picture_url: input.picture_url.clone(),
        text: input.text.clone(),
        channel_id: input.channel_id,
        guild_id: input.guild_id,
    };

    let collection = db.collection::<Snipe>("snipes");
    let insert_result = collection
        .insert_one(&snipe, None)
        .await
        .map_err(|e| ApiError::internal_error(format!("Failed to create snipe: {}", e)))?;

    // Fetch the created snipe with its MongoDB _id
    let created_snipe = collection
        .find_one(doc! { "_id": insert_result.inserted_id }, None)
        .await
        .map_err(|e| ApiError::internal_error(format!("Failed to fetch created snipe: {}", e)))?
        .ok_or_else(|| ApiError::internal_error("Snipe was created but not found".to_string()))?;

    Ok(Json(ApiResponse::success(created_snipe)))
}

/// PUT /api/snipes/:object_id
/// Update a snipe's information
#[put("/snipes/<object_id>", data = "<input>")]
pub async fn update_snipe(
    db: &State<Database>,
    object_id: String,
    input: Json<UpdateSnipeInput>,
) -> Result<Json<ApiResponse<Snipe>>, ApiError> {
    let oid = ObjectId::parse_str(&object_id)
        .map_err(|_| ApiError::bad_request("Invalid ObjectId format".to_string()))?;

    let collection = db.collection::<Snipe>("snipes");

    // Check snipe exists
    collection
        .find_one(doc! { "_id": &oid }, None)
        .await
        .map_err(|e| ApiError::internal_error(format!("Database error: {}", e)))?
        .ok_or_else(|| ApiError::not_found(format!("Snipe with id {} not found", object_id)))?;

    // Build update document
    let mut update_doc = mongodb::bson::Document::new();
    if let Some(picture_url) = &input.picture_url {
        if picture_url.is_empty() {
            return Err(ApiError::validation_error(
                "Picture URL cannot be empty".to_string(),
            ));
        }
        update_doc.insert("picture_url", picture_url);
    }
    if let Some(text) = &input.text {
        update_doc.insert("text", text);
    }

    if update_doc.is_empty() {
        return Err(ApiError::bad_request("No fields to update".to_string()));
    }

    collection
        .update_one(doc! { "_id": &oid }, doc! { "$set": update_doc }, None)
        .await
        .map_err(|e| ApiError::internal_error(format!("Failed to update snipe: {}", e)))?;

    // Fetch updated snipe
    let updated_snipe = collection
        .find_one(doc! { "_id": &oid }, None)
        .await
        .map_err(|e| ApiError::internal_error(format!("Failed to fetch updated snipe: {}", e)))?
        .ok_or_else(|| ApiError::internal_error("Snipe was updated but not found".to_string()))?;

    Ok(Json(ApiResponse::success(updated_snipe)))
}

/// DELETE /api/snipes/:object_id
/// Delete a specific snipe
#[delete("/snipes/<object_id>")]
pub async fn delete_snipe(
    db: &State<Database>,
    object_id: String,
) -> Result<Json<ApiResponse<String>>, ApiError> {
    let oid = ObjectId::parse_str(&object_id)
        .map_err(|_| ApiError::bad_request("Invalid ObjectId format".to_string()))?;

    let collection = db.collection::<Snipe>("snipes");

    // Check snipe exists
    collection
        .find_one(doc! { "_id": &oid }, None)
        .await
        .map_err(|e| ApiError::internal_error(format!("Database error: {}", e)))?
        .ok_or_else(|| ApiError::not_found(format!("Snipe with id {} not found", object_id)))?;

    // Delete the snipe
    collection
        .delete_one(doc! { "_id": &oid }, None)
        .await
        .map_err(|e| ApiError::internal_error(format!("Failed to delete snipe: {}", e)))?;

    Ok(Json(ApiResponse::success(format!(
        "Snipe with id {} deleted successfully",
        object_id
    ))))
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
    use crate::models::Person;
    use mongodb::bson::doc;
    use rocket::local::blocking::Client;
    use rocket::http::{Status, ContentType};

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
                    list_snipes,
                    get_snipe,
                    get_snipes_by_channel,
                    get_snipes_by_guild,
                    search_snipes,
                    create_snipe,
                    update_snipe,
                    delete_snipe,
                ],
            )
    }

    fn setup_test_persons(db: &Database) {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let collection = db.collection::<Person>("persons");
            collection.delete_many(doc! { "id": { "$in": [888888888, 777777777] } }, None).await.ok();
            
            collection.insert_one(Person {
                object_id: None,
                id: 888888888,
                username: "sniper_test".to_string(),
                display_name: Some("Sniper".to_string()),
            }, None).await.ok();
            
            collection.insert_one(Person {
                object_id: None,
                id: 777777777,
                username: "snipee_test".to_string(),
                display_name: Some("Snipee".to_string()),
            }, None).await.ok();
        });
    }

    fn cleanup_test_data(db: &Database) {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let snipes_collection = db.collection::<Snipe>("snipes");
            snipes_collection.delete_many(
                doc! { "sniper_id": { "$in": [888888888, 777777777] } }, 
                None
            ).await.ok();
            
            let persons_collection = db.collection::<Person>("persons");
            persons_collection.delete_many(
                doc! { "id": { "$in": [888888888, 777777777] } }, 
                None
            ).await.ok();
        });
    }

    #[test]
    fn test_create_and_get_snipe() {
        if !check_mongodb_available() {
            eprintln!("⚠️  Skipping test: MongoDB not available at localhost:27017");
            eprintln!("   Start MongoDB with: docker run -d -p 27017:27017 mongo");
            return;
        }
        let db = get_test_db();
        setup_test_persons(&db);

        let client = Client::tracked(setup_rocket(db.clone())).expect("valid rocket instance");

        // Create a snipe
        let response = client
            .post("/api/snipes")
            .header(ContentType::JSON)
            .body(r#"{
                "sniper_id": 888888888,
                "snipee_id": 777777777,
                "picture_url": "https://test.com/image.jpg",
                "text": "Test snipe",
                "channel_id": 123456,
                "guild_id": 789012
            }"#)
            .dispatch();

        assert_eq!(response.status(), Status::Ok);
        let body = response.into_string().unwrap();
        assert!(body.contains("Test snipe"));
        assert!(body.contains("_id"));

        // Extract the ObjectId from the response
        let json: serde_json::Value = serde_json::from_str(&body).unwrap();
        let object_id = json["data"]["_id"]["$oid"].as_str().unwrap();

        // Get the snipe
        let response = client.get(format!("/api/snipes/{}", object_id)).dispatch();
        assert_eq!(response.status(), Status::Ok);

        cleanup_test_data(&db);
    }

    #[test]
    fn test_list_snipes() {
        if !check_mongodb_available() {
            eprintln!("⚠️  Skipping test: MongoDB not available at localhost:27017");
            eprintln!("   Start MongoDB with: docker run -d -p 27017:27017 mongo");
            return;
        }
        let db = get_test_db();
        let client = Client::tracked(setup_rocket(db.clone())).expect("valid rocket instance");

        let response = client.get("/api/snipes").dispatch();
        assert_eq!(response.status(), Status::Ok);
        let body = response.into_string().unwrap();
        assert!(body.contains("\"success\":true"));
        assert!(body.contains("\"meta\""));
    }

    #[test]
    fn test_get_snipes_by_channel() {
        if !check_mongodb_available() {
            eprintln!("⚠️  Skipping test: MongoDB not available at localhost:27017");
            eprintln!("   Start MongoDB with: docker run -d -p 27017:27017 mongo");
            return;
        }
        let db = get_test_db();
        setup_test_persons(&db);

        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let collection = db.collection::<Snipe>("snipes");
            collection.insert_one(Snipe {
                object_id: None,
                sniper_id: 888888888,
                snipee_id: 777777777,
                picture_url: "https://test.com/image.jpg".to_string(),
                text: Some("Test".to_string()),
                channel_id: 111222333,
                guild_id: Some(444555666),
            }, None).await.ok();
        });

        let client = Client::tracked(setup_rocket(db.clone())).expect("valid rocket instance");

        let response = client.get("/api/snipes/by-channel/111222333").dispatch();
        assert_eq!(response.status(), Status::Ok);
        let body = response.into_string().unwrap();
        assert!(body.contains("\"channel_id\":111222333"));

        cleanup_test_data(&db);
    }

    #[test]
    fn test_get_snipes_by_guild() {
        if !check_mongodb_available() {
            eprintln!("⚠️  Skipping test: MongoDB not available at localhost:27017");
            eprintln!("   Start MongoDB with: docker run -d -p 27017:27017 mongo");
            return;
        }
        let db = get_test_db();
        setup_test_persons(&db);

        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let collection = db.collection::<Snipe>("snipes");
            collection.insert_one(Snipe {
                object_id: None,
                sniper_id: 888888888,
                snipee_id: 777777777,
                picture_url: "https://test.com/image.jpg".to_string(),
                text: Some("Test".to_string()),
                channel_id: 111222333,
                guild_id: Some(444555666),
            }, None).await.ok();
        });

        let client = Client::tracked(setup_rocket(db.clone())).expect("valid rocket instance");

        let response = client.get("/api/snipes/by-guild/444555666").dispatch();
        assert_eq!(response.status(), Status::Ok);

        cleanup_test_data(&db);
    }

    #[test]
    fn test_search_snipes() {
        if !check_mongodb_available() {
            eprintln!("⚠️  Skipping test: MongoDB not available at localhost:27017");
            eprintln!("   Start MongoDB with: docker run -d -p 27017:27017 mongo");
            return;
        }
        let db = get_test_db();
        setup_test_persons(&db);

        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let collection = db.collection::<Snipe>("snipes");
            collection.insert_one(Snipe {
                object_id: None,
                sniper_id: 888888888,
                snipee_id: 777777777,
                picture_url: "https://test.com/image.jpg".to_string(),
                text: Some("Test".to_string()),
                channel_id: 111222333,
                guild_id: Some(444555666),
            }, None).await.ok();
        });

        let client = Client::tracked(setup_rocket(db.clone())).expect("valid rocket instance");

        // Search by sniper
        let response = client.get("/api/snipes/search?sniper_id=888888888").dispatch();
        assert_eq!(response.status(), Status::Ok);

        // Search by has_text
        let response = client.get("/api/snipes/search?has_text=true").dispatch();
        assert_eq!(response.status(), Status::Ok);

        cleanup_test_data(&db);
    }

    #[test]
    fn test_create_snipe_missing_person() {
        if !check_mongodb_available() {
            eprintln!("⚠️  Skipping test: MongoDB not available at localhost:27017");
            eprintln!("   Start MongoDB with: docker run -d -p 27017:27017 mongo");
            return;
        }
        let db = get_test_db();
        let client = Client::tracked(setup_rocket(db.clone())).expect("valid rocket instance");

        // Try to create snipe with non-existent persons
        let response = client
            .post("/api/snipes")
            .header(ContentType::JSON)
            .body(r#"{
                "sniper_id": 111111111,
                "snipee_id": 222222222,
                "picture_url": "https://test.com/image.jpg",
                "text": "Test",
                "channel_id": 123456,
                "guild_id": 789012
            }"#)
            .dispatch();

        assert_eq!(response.status(), Status::UnprocessableEntity);
        let body = response.into_string().unwrap();
        assert!(body.contains("VALIDATION_ERROR"));
    }

    #[test]
    fn test_update_snipe() {
        if !check_mongodb_available() {
            eprintln!("⚠️  Skipping test: MongoDB not available at localhost:27017");
            eprintln!("   Start MongoDB with: docker run -d -p 27017:27017 mongo");
            return;
        }
        let db = get_test_db();
        setup_test_persons(&db);

        let rt = tokio::runtime::Runtime::new().unwrap();
        let object_id = rt.block_on(async {
            let collection = db.collection::<Snipe>("snipes");
            let result = collection.insert_one(Snipe {
                object_id: None,
                sniper_id: 888888888,
                snipee_id: 777777777,
                picture_url: "https://old-url.com/image.jpg".to_string(),
                text: Some("Old text".to_string()),
                channel_id: 111222333,
                guild_id: Some(444555666),
            }, None).await.unwrap();
            result.inserted_id.as_object_id().unwrap().to_hex()
        });

        let client = Client::tracked(setup_rocket(db.clone())).expect("valid rocket instance");

        let response = client
            .put(format!("/api/snipes/{}", object_id))
            .header(ContentType::JSON)
            .body(r#"{"text": "Updated text"}"#)
            .dispatch();

        assert_eq!(response.status(), Status::Ok);
        let body = response.into_string().unwrap();
        assert!(body.contains("Updated text"));

        cleanup_test_data(&db);
    }

    #[test]
    fn test_delete_snipe() {
        if !check_mongodb_available() {
            eprintln!("⚠️  Skipping test: MongoDB not available at localhost:27017");
            eprintln!("   Start MongoDB with: docker run -d -p 27017:27017 mongo");
            return;
        }
        let db = get_test_db();
        setup_test_persons(&db);

        let rt = tokio::runtime::Runtime::new().unwrap();
        let object_id = rt.block_on(async {
            let collection = db.collection::<Snipe>("snipes");
            let result = collection.insert_one(Snipe {
                object_id: None,
                sniper_id: 888888888,
                snipee_id: 777777777,
                picture_url: "https://test.com/image.jpg".to_string(),
                text: Some("To delete".to_string()),
                channel_id: 111222333,
                guild_id: Some(444555666),
            }, None).await.unwrap();
            result.inserted_id.as_object_id().unwrap().to_hex()
        });

        let client = Client::tracked(setup_rocket(db.clone())).expect("valid rocket instance");

        let response = client.delete(format!("/api/snipes/{}", object_id)).dispatch();
        assert_eq!(response.status(), Status::Ok);

        // Verify deletion
        let response = client.get(format!("/api/snipes/{}", object_id)).dispatch();
        assert_eq!(response.status(), Status::NotFound);

        cleanup_test_data(&db);
    }

    #[test]
    fn test_invalid_object_id() {
        if !check_mongodb_available() {
            eprintln!("⚠️  Skipping test: MongoDB not available at localhost:27017");
            eprintln!("   Start MongoDB with: docker run -d -p 27017:27017 mongo");
            return;
        }
        let db = get_test_db();
        let client = Client::tracked(setup_rocket(db)).expect("valid rocket instance");

        let response = client.get("/api/snipes/invalid_id").dispatch();
        assert_eq!(response.status(), Status::BadRequest);
        let body = response.into_string().unwrap();
        assert!(body.contains("Invalid ObjectId"));
    }
}
