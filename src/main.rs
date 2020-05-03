use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use sqlite::open;
use std::fs::File;
use std::io::prelude::*;
use rand::seq::SliceRandom;
use std::time::Instant;
use std::path::Path;
use std::io::{BufReader, Error};

async fn post_url(long_url: web::Path<String>) -> impl Responder {
    let connection = sqlite::open(":memory:").unwrap();
    let shortened_url = generate_url();
    let mut statement = connection
        .prepare("INSERT INTO url (long, short) VALUES (?, ?);")
        .unwrap();

    statement.bind(1, long_url.as_bytes()).unwrap();
    statement.bind(2, shortened_url.as_bytes()).unwrap();

    HttpResponse::Ok().body("ok.")
}

async fn get_url() -> impl Responder {
    let connection = sqlite::open(":memory:").unwrap();
    HttpResponse::Ok().body("")
}

fn generate_url() -> String {
    let path = Path::new("src/words.txt");
    let display = path.display();
    println!("{:?}", display);
    match File::open(&path) {
        Err(error) => {
            panic!("{:?}", error)
        }
        Ok(file) => {
            let mut copy = file.try_clone().unwrap();
            let reader = BufReader::new(copy);
            let mut content: Vec<String> = vec![];
            for line in reader.lines() {
                content.push(line.unwrap());
            }

            let mut string_builder = String::new();
            for i in 0..3 {
                string_builder.push_str(content.choose(&mut rand::thread_rng()).unwrap());
            }

            println!("{}", string_builder);
            return string_builder;
        }
    };
}

fn create_database_schema() {
    println!("Initializing database...");
    let start = Instant::now();
    let connection = sqlite::open(":memory:").unwrap();
    connection
        .execute("CREATE TABLE url(id INTEGER PRIMARY KEY AUTOINCREMENT, long TEXT NOT NULL, short TEXT NOT NULL);")
        .unwrap();
    let end = Instant::now();
    let duration = end.duration_since(start);
    println!("Database initialized in {:?}!", duration);
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