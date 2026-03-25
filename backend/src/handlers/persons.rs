use mongodb::bson::doc;
use mongodb::Database;
use rocket::serde::json::Json;
use rocket::State;

use crate::models::{CreatePersonInput, Person, PersonStats, UpdatePersonInput};
use crate::responses::{ApiError, ApiResponse, PaginationParams, ResponseMeta};

/// GET /api/persons?limit=50&offset=0
/// List all persons with pagination
#[get("/persons?<pagination..>")]
pub async fn list_persons(
    db: &State<Database>,
    pagination: PaginationParams,
) -> Result<Json<ApiResponse<Vec<Person>>>, ApiError> {
    pagination.validate()?;

    let collection = db.collection::<Person>("persons");

    // Get total count
    let total = collection
        .count_documents(None, None)
        .await
        .map_err(|e| ApiError::internal_error(format!("Failed to count persons: {}", e)))?;

    // Get paginated results
    let options = mongodb::options::FindOptions::builder()
        .skip(Some(pagination.offset as u64))
        .limit(Some(pagination.limit))
        .build();

    let mut cursor = collection
        .find(None, options)
        .await
        .map_err(|e| ApiError::internal_error(format!("Failed to fetch persons: {}", e)))?;

    let mut persons = Vec::new();
    use futures::stream::StreamExt;
    while let Some(result) = cursor.next().await {
        match result {
            Ok(person) => persons.push(person),
            Err(e) => {
                return Err(ApiError::internal_error(format!(
                    "Failed to read person: {}",
                    e
                )))
            }
        }
    }

    let meta = ResponseMeta::new(total as i64, pagination.limit, pagination.offset);
    Ok(Json(ApiResponse::with_meta(persons, meta)))
}

/// GET /api/persons/:id
/// Get a specific person by Discord ID
#[get("/persons/<id>")]
pub async fn get_person(db: &State<Database>, id: i64) -> Result<Json<ApiResponse<Person>>, ApiError> {
    let collection = db.collection::<Person>("persons");

    let person = collection
        .find_one(doc! { "id": id }, None)
        .await
        .map_err(|e| ApiError::internal_error(format!("Database error: {}", e)))?
        .ok_or_else(|| ApiError::not_found(format!("Person with id {} not found", id)))?;

    Ok(Json(ApiResponse::success(person)))
}

/// GET /api/persons/:id/snipes-by?limit=50&offset=0
/// Get all snipes taken BY this person
#[get("/persons/<id>/snipes-by?<pagination..>")]
pub async fn get_snipes_by_person(
    db: &State<Database>,
    id: i64,
    pagination: PaginationParams,
) -> Result<Json<ApiResponse<Vec<crate::models::Snipe>>>, ApiError> {
    pagination.validate()?;

    // Check person exists
    let persons_collection = db.collection::<Person>("persons");
    persons_collection
        .find_one(doc! { "id": id }, None)
        .await
        .map_err(|e| ApiError::internal_error(format!("Database error: {}", e)))?
        .ok_or_else(|| ApiError::not_found(format!("Person with id {} not found", id)))?;

    let snipes_collection = db.collection::<crate::models::Snipe>("snipes");

    // Get total count
    let filter = doc! { "sniper_id": id };
    let total = snipes_collection
        .count_documents(filter.clone(), None)
        .await
        .map_err(|e| ApiError::internal_error(format!("Failed to count snipes: {}", e)))?;

    // Get paginated results
    let options = mongodb::options::FindOptions::builder()
        .skip(Some(pagination.offset as u64))
        .limit(Some(pagination.limit))
        .build();

    let mut cursor = snipes_collection
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

/// GET /api/persons/:id/snipes-of?limit=50&offset=0
/// Get all snipes OF this person (where they are the snipee)
#[get("/persons/<id>/snipes-of?<pagination..>")]
pub async fn get_snipes_of_person(
    db: &State<Database>,
    id: i64,
    pagination: PaginationParams,
) -> Result<Json<ApiResponse<Vec<crate::models::Snipe>>>, ApiError> {
    pagination.validate()?;

    // Check person exists
    let persons_collection = db.collection::<Person>("persons");
    persons_collection
        .find_one(doc! { "id": id }, None)
        .await
        .map_err(|e| ApiError::internal_error(format!("Database error: {}", e)))?
        .ok_or_else(|| ApiError::not_found(format!("Person with id {} not found", id)))?;

    let snipes_collection = db.collection::<crate::models::Snipe>("snipes");

    // Get total count
    let filter = doc! { "snipee_id": id };
    let total = snipes_collection
        .count_documents(filter.clone(), None)
        .await
        .map_err(|e| ApiError::internal_error(format!("Failed to count snipes: {}", e)))?;

    // Get paginated results
    let options = mongodb::options::FindOptions::builder()
        .skip(Some(pagination.offset as u64))
        .limit(Some(pagination.limit))
        .build();

    let mut cursor = snipes_collection
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

/// GET /api/persons/:id/stats
/// Get statistics for a person
#[get("/persons/<id>/stats")]
pub async fn get_person_stats(
    db: &State<Database>,
    id: i64,
) -> Result<Json<ApiResponse<PersonStats>>, ApiError> {
    let persons_collection = db.collection::<Person>("persons");

    // Get person
    let person = persons_collection
        .find_one(doc! { "id": id }, None)
        .await
        .map_err(|e| ApiError::internal_error(format!("Database error: {}", e)))?
        .ok_or_else(|| ApiError::not_found(format!("Person with id {} not found", id)))?;

    let snipes_collection = db.collection::<crate::models::Snipe>("snipes");

    // Count snipes taken
    let snipes_taken = snipes_collection
        .count_documents(doc! { "sniper_id": id }, None)
        .await
        .map_err(|e| ApiError::internal_error(format!("Failed to count snipes taken: {}", e)))?;

    // Count snipes received
    let snipes_received = snipes_collection
        .count_documents(doc! { "snipee_id": id }, None)
        .await
        .map_err(|e| {
            ApiError::internal_error(format!("Failed to count snipes received: {}", e))
        })?;

    let stats = PersonStats {
        person,
        snipes_taken: snipes_taken as i64,
        snipes_received: snipes_received as i64,
    };

    Ok(Json(ApiResponse::success(stats)))
}

/// POST /api/persons
/// Create a new person
#[post("/persons", data = "<input>")]
pub async fn create_person(
    db: &State<Database>,
    input: Json<CreatePersonInput>,
) -> Result<Json<ApiResponse<Person>>, ApiError> {
    let collection = db.collection::<Person>("persons");

    // Check if person already exists
    let existing = collection
        .find_one(doc! { "id": input.id }, None)
        .await
        .map_err(|e| ApiError::internal_error(format!("Database error: {}", e)))?;

    if existing.is_some() {
        return Err(ApiError::conflict(format!(
            "Person with id {} already exists",
            input.id
        )));
    }

    // Validate input
    if input.username.is_empty() {
        return Err(ApiError::validation_error(
            "Username cannot be empty".to_string(),
        ));
    }

    let person = Person {
        object_id: None,
        id: input.id,
        username: input.username.clone(),
        display_name: input.display_name.clone(),
    };

    let insert_result = collection
        .insert_one(&person, None)
        .await
        .map_err(|e| ApiError::internal_error(format!("Failed to create person: {}", e)))?;

    // Fetch the created person with its MongoDB _id
    let created_person = collection
        .find_one(doc! { "_id": insert_result.inserted_id }, None)
        .await
        .map_err(|e| ApiError::internal_error(format!("Failed to fetch created person: {}", e)))?
        .ok_or_else(|| ApiError::internal_error("Person was created but not found".to_string()))?;

    Ok(Json(ApiResponse::success(created_person)))
}

/// PUT /api/persons/:id
/// Update a person's information
#[put("/persons/<id>", data = "<input>")]
pub async fn update_person(
    db: &State<Database>,
    id: i64,
    input: Json<UpdatePersonInput>,
) -> Result<Json<ApiResponse<Person>>, ApiError> {
    let collection = db.collection::<Person>("persons");

    // Check person exists
    collection
        .find_one(doc! { "id": id }, None)
        .await
        .map_err(|e| ApiError::internal_error(format!("Database error: {}", e)))?
        .ok_or_else(|| ApiError::not_found(format!("Person with id {} not found", id)))?;

    // Build update document
    let mut update_doc = mongodb::bson::Document::new();
    if let Some(username) = &input.username {
        if username.is_empty() {
            return Err(ApiError::validation_error(
                "Username cannot be empty".to_string(),
            ));
        }
        update_doc.insert("username", username);
    }
    if let Some(display_name) = &input.display_name {
        update_doc.insert("display_name", display_name);
    }

    if update_doc.is_empty() {
        return Err(ApiError::bad_request("No fields to update".to_string()));
    }

    collection
        .update_one(doc! { "id": id }, doc! { "$set": update_doc }, None)
        .await
        .map_err(|e| ApiError::internal_error(format!("Failed to update person: {}", e)))?;

    // Fetch updated person
    let updated_person = collection
        .find_one(doc! { "id": id }, None)
        .await
        .map_err(|e| ApiError::internal_error(format!("Failed to fetch updated person: {}", e)))?
        .ok_or_else(|| ApiError::internal_error("Person was updated but not found".to_string()))?;

    Ok(Json(ApiResponse::success(updated_person)))
}

/// DELETE /api/persons/:id?cascade=false
/// Delete a person (and optionally their snipes)
#[delete("/persons/<id>?<cascade>")]
pub async fn delete_person(
    db: &State<Database>,
    id: i64,
    cascade: Option<bool>,
) -> Result<Json<ApiResponse<String>>, ApiError> {
    let persons_collection = db.collection::<Person>("persons");

    // Check person exists
    persons_collection
        .find_one(doc! { "id": id }, None)
        .await
        .map_err(|e| ApiError::internal_error(format!("Database error: {}", e)))?
        .ok_or_else(|| ApiError::not_found(format!("Person with id {} not found", id)))?;

    // If cascade is true, delete all snipes involving this person
    if cascade.unwrap_or(false) {
        let snipes_collection = db.collection::<crate::models::Snipe>("snipes");
        snipes_collection
            .delete_many(
                doc! {
                    "$or": [
                        { "sniper_id": id },
                        { "snipee_id": id }
                    ]
                },
                None,
            )
            .await
            .map_err(|e| {
                ApiError::internal_error(format!("Failed to delete related snipes: {}", e))
            })?;
    }

    // Delete the person
    persons_collection
        .delete_one(doc! { "id": id }, None)
        .await
        .map_err(|e| ApiError::internal_error(format!("Failed to delete person: {}", e)))?;

    Ok(Json(ApiResponse::success(format!(
        "Person with id {} deleted successfully",
        id
    ))))
}

#[cfg(test)]
mod tests {
    use super::*;
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

    fn setup_rocket(db: Database) -> rocket::Rocket<rocket::Build> {
        rocket::build()
            .manage(db)
            .mount(
                "/api",
                routes![
                    list_persons,
                    get_person,
                    get_snipes_by_person,
                    get_snipes_of_person,
                    get_person_stats,
                    create_person,
                    update_person,
                    delete_person,
                ],
            )
    }

    #[test]
    fn test_create_and_get_person() {
        if !check_mongodb_available() {
            eprintln!("⚠️  Skipping test: MongoDB not available at localhost:27017");
            eprintln!("   Start MongoDB with: docker run -d -p 27017:27017 mongo");
            return;
        }

        let db = get_test_db();
        
        // Clean up test data
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let collection = db.collection::<Person>("persons");
            collection.delete_many(doc! { "id": 999999999 }, None).await.ok();
        });

        let client = Client::tracked(setup_rocket(db.clone())).expect("valid rocket instance");

        // Create a person
        let response = client
            .post("/api/persons")
            .header(ContentType::JSON)
            .body(r#"{"id": 999999999, "username": "test_user", "display_name": "Test User"}"#)
            .dispatch();

        let status = response.status();
        let body = response.into_string().unwrap();
        if status != Status::Ok {
            eprintln!("Error response: {}", body);
        }
        assert_eq!(status, Status::Ok, "Response body: {}", body);
        assert!(body.contains("test_user"));

        // Get the person
        let response = client.get("/api/persons/999999999").dispatch();
        assert_eq!(response.status(), Status::Ok);
        let body = response.into_string().unwrap();
        assert!(body.contains("test_user"));

        // Clean up
        rt.block_on(async {
            let collection = db.collection::<Person>("persons");
            collection.delete_many(doc! { "id": 999999999 }, None).await.ok();
        });
    }

    #[test]
    fn test_list_persons_pagination() {
        let db = get_test_db();
        let client = Client::tracked(setup_rocket(db.clone())).expect("valid rocket instance");

        // Test with default pagination
        let response = client.get("/api/persons").dispatch();
        assert_eq!(response.status(), Status::Ok);
        let body = response.into_string().unwrap();
        assert!(body.contains("\"success\":true"));
        assert!(body.contains("\"meta\""));

        // Test with custom pagination
        let response = client.get("/api/persons?limit=10&offset=0").dispatch();
        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn test_get_nonexistent_person() {
        let db = get_test_db();
        let client = Client::tracked(setup_rocket(db)).expect("valid rocket instance");

        let response = client.get("/api/persons/111111111").dispatch();
        assert_eq!(response.status(), Status::NotFound);
        let body = response.into_string().unwrap();
        assert!(body.contains("\"success\":false"));
        assert!(body.contains("NOT_FOUND"));
    }

    #[test]
    fn test_update_person() {
        let db = get_test_db();
        let rt = tokio::runtime::Runtime::new().unwrap();
        
        // Create test person
        rt.block_on(async {
            let collection = db.collection::<Person>("persons");
            collection.delete_many(doc! { "id": 999999998 }, None).await.ok();
            collection.insert_one(Person {
                object_id: None,
                id: 999999998,
                username: "old_name".to_string(),
                display_name: Some("Old".to_string()),
            }, None).await.ok();
        });

        let client = Client::tracked(setup_rocket(db.clone())).expect("valid rocket instance");

        // Update the person
        let response = client
            .put("/api/persons/999999998")
            .header(ContentType::JSON)
            .body(r#"{"username": "new_name"}"#)
            .dispatch();

        assert_eq!(response.status(), Status::Ok);
        let body = response.into_string().unwrap();
        assert!(body.contains("new_name"));

        // Clean up
        rt.block_on(async {
            let collection = db.collection::<Person>("persons");
            collection.delete_many(doc! { "id": 999999998 }, None).await.ok();
        });
    }

    #[test]
    fn test_delete_person() {
        let db = get_test_db();
        let rt = tokio::runtime::Runtime::new().unwrap();
        
        // Create test person
        rt.block_on(async {
            let collection = db.collection::<Person>("persons");
            collection.delete_many(doc! { "id": 999999997 }, None).await.ok();
            collection.insert_one(Person {
                object_id: None,
                id: 999999997,
                username: "to_delete".to_string(),
                display_name: None,
            }, None).await.ok();
        });

        let client = Client::tracked(setup_rocket(db.clone())).expect("valid rocket instance");

        // Delete the person
        let response = client.delete("/api/persons/999999997").dispatch();
        assert_eq!(response.status(), Status::Ok);

        // Verify deletion
        let response = client.get("/api/persons/999999997").dispatch();
        assert_eq!(response.status(), Status::NotFound);
    }

    #[test]
    fn test_create_duplicate_person() {
        let db = get_test_db();
        let rt = tokio::runtime::Runtime::new().unwrap();
        
        // Create test person
        rt.block_on(async {
            let collection = db.collection::<Person>("persons");
            collection.delete_many(doc! { "id": 999999996 }, None).await.ok();
            collection.insert_one(Person {
                object_id: None,
                id: 999999996,
                username: "existing".to_string(),
                display_name: None,
            }, None).await.ok();
        });

        let client = Client::tracked(setup_rocket(db.clone())).expect("valid rocket instance");

        // Try to create duplicate
        let response = client
            .post("/api/persons")
            .header(ContentType::JSON)
            .body(r#"{"id": 999999996, "username": "duplicate", "display_name": null}"#)
            .dispatch();

        assert_eq!(response.status(), Status::Conflict);
        let body = response.into_string().unwrap();
        assert!(body.contains("CONFLICT"));

        // Clean up
        rt.block_on(async {
            let collection = db.collection::<Person>("persons");
            collection.delete_many(doc! { "id": 999999996 }, None).await.ok();
        });
    }

    #[test]
    fn test_invalid_pagination() {
        let db = get_test_db();
        let client = Client::tracked(setup_rocket(db)).expect("valid rocket instance");

        // Test limit too high
        let response = client.get("/api/persons?limit=200").dispatch();
        assert_eq!(response.status(), Status::BadRequest);

        // Test negative offset
        let response = client.get("/api/persons?offset=-1").dispatch();
        assert_eq!(response.status(), Status::BadRequest);
    }
}
