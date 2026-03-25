#[macro_use]
extern crate rocket;

use rocket::fs::{FileServer, relative};
use std::path::Path;
use rocket::tokio::fs::File;
use rocket::tokio::io::AsyncReadExt;
use rocket::http::ContentType;
use rocket::State;
use std::env;
use rocket::tokio;
use mongodb::{Client, Database};

mod discord_bot;
mod models;

#[get("/test_db")]
async fn test_db(db: &State<Database>) -> String {
    // Run a basic MongoDB command: list collections
    match db.list_collection_names(None).await {
        Ok(list) => format!("MongoDB connection succeeded! Collections: {:?}", list),
        Err(e) => format!("MongoDB connection failed: {e}"),
    }
}

#[get("/<_..>", rank = 20)]
async fn spa_fallback() -> Option<(ContentType, String)> {
    let index_path = Path::new(relative!("static")).join("index.html");
    let mut file = File::open(index_path).await.ok()?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await.ok()?;
    Some((ContentType::HTML, contents))
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    dotenvy::dotenv().ok(); // Load .env file on startup
    let mongo_uri = env::var("MONGODB_URI").expect("MONGODB_URI missing in .env");
    let db_name = env::var("MONGODB_DB").expect("MONGODB_DB missing in .env");
    let discord_token = env::var("DISCORD_BOT_TOKEN").unwrap_or_default();

    // Create MongoDB Client and get Database handle
    let client = Client::with_uri_str(&mongo_uri).await.expect("Failed to connect to MongoDB");
    let db = client.database(&db_name);

    // Pass database handle to Discord bot in the background
    let discord_db = db.clone();
    tokio::spawn(async move {
        crate::discord_bot::start_discord_bot(discord_db, discord_token).await;
    });

    let rocket = rocket::build()
        .manage(db)
        .mount("/api", routes![test_db])
        .mount("/api", routes![])
        .mount("/", FileServer::from(relative!("static")).rank(0))
        .mount("/", routes![spa_fallback]);
    rocket.launch().await?;
    Ok(())
}
