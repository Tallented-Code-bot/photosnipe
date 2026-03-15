#[macro_use]
extern crate rocket;

use rocket::fs::{FileServer, relative};
use rocket::response::content::RawHtml;
use std::path::{Path, PathBuf};
use rocket::tokio::fs::File;
use rocket::tokio::io::AsyncReadExt;
use rocket::http::ContentType;

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
        .mount("/api", routes![]) // All API routes will be here in future
        .mount("/", FileServer::from(relative!("static")).rank(0))
        .mount("/", routes![spa_fallback])
}
