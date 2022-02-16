extern crate dotenv;

use dotenv::dotenv;
use std::env;

use minestom_servers::database::establish_database_pool;

use actix_web::{
    HttpServer, 
    App, 
    HttpResponse,
    Responder,
    get,
    post,
};

#[get("/")]
async fn home() -> impl Responder {
    HttpResponse::Ok().body("Hello World!")
}

#[post("/")]
async fn post_home() -> impl Responder {
    HttpResponse::Ok().body("Hello Poster!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load the environment variables
    dotenv().ok();

    // Getting the database information and creating the pool (must be postgres)
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let database_pool = establish_database_pool(&database_url);

    // Getting the address to bind to
    let host = env::var("HOST").unwrap_or(String::from("127.0.0.1"));
    let port = env::var("PORT").unwrap_or(String::from("8080"));
    let addr = format!("{}:{}", host, port);

    println!("Starting API bound to {}", addr);

    // Actually starting the web API
    HttpServer::new(move || {
        App::new()
            .data(database_pool.clone())
            .service(home)
    })
    .bind(&addr)?
    .run()
    .await
}
