#[macro_use]
extern crate rocket;

use rocket::fs::{FileServer, relative};
use rocket::response::content::RawHtml;
use std::path::{Path, PathBuf};
use rocket::tokio::fs::File;
use rocket::tokio::io::AsyncReadExt;
use rocket::http::ContentType;

// --- Database setup ---
use rocket_db_pools::{Database, Connection};

#[derive(Database)]
#[database("photosnipe_db")]
pub struct Db(sqlx::PgPool);

// --- Example test route using DB ---
#[get("/test_db")]
async fn test_db(mut db: Connection<Db>) -> String {
    // Generic test query: SELECT 1 as n
    let row: (i32,) = match sqlx::query_as("SELECT 1 as n")
        .fetch_one(&mut **db).await {
        Ok(r) => r,
        Err(e) => return format!("Database connection failed: {e}"),
    };
    format!("Test query succeeded: n = {}", row.0)
}

#[get("/<_..>", rank = 20)]
async fn spa_fallback() -> Option<(ContentType, String)> {
    // Serve index.html for any GET request that doesn't match an API or static file
    let index_path = Path::new(relative!("static")).join("index.html");
    let mut file = File::open(index_path).await.ok()?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await.ok()?;
    Some((ContentType::HTML, contents))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Db::init())
        .mount("/api", routes![test_db])
        .mount("/api", routes![]) // All API routes will be here in future
        .mount("/", FileServer::from(relative!("static")).rank(0))
        .mount("/", routes![spa_fallback])
}
