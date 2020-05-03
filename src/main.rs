use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use sqlite::open;
use std::fs::File;
use std::io::prelude::*;
use rand::seq::SliceRandom;

async fn post_url(long_url: web::Path<String>) -> impl Responder {
    let connection = sqlite::open(":memory:").unwrap();
    let shortened_url = generate_url();
    let mut statement = connection
        .prepare("INSERT INTO url (long, short) VALUES (?, ?);").unwrap();

    statement.bind(1, long_url.as_bytes()).unwrap();
    statement.bind(2, shortened_url).unwrap();

    HttpResponse::Ok().body("ok.")
}

async fn get_url() -> impl Responder {
    let connection = sqlite::open(":memory:").unwrap();
    HttpResponse::Ok().body("")
}

fn generate_url() {
    let filename = "words.txt";
}

fn create_database_schema() {
    // db is in memory - haHAA - no drops
    println!("Initializing database...");
    let connection = sqlite::open(":memory:").unwrap();
    connection
        .execute("CREATE TABLE url(id INT PRIMARY KEY AUTOINCREMENT NOT NULL, long TEXT NOT NULL, short TEXT NOT NULL);")
        .unwrap();
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    create_database_schema();
    HttpServer::new(|| {
        App::new()
            .route("/{long_url}", web::post().to(post_url))
            .route("/{long_url}", web::get().to(get_url))
    })
        .bind("127.0.0.1:8088")?
        .run()
        .await
}