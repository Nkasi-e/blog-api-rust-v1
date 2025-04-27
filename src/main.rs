use actix_web::{web, middleware::Logger, App, HttpServer };
use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};
use utils::app_state::AppState;
mod utils;
mod routes;


#[derive(Debug)]
struct MainError {
    message: String
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> Result<(), MainError> {

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }

    dotenv::dotenv().ok();
    env_logger::init(); // logs all the information of the coming request

    let port: u16 = (*utils::constants::PORT).clone();
    let address: String = (*utils::constants::ADDRESS).clone();
    let database_url: String = (*utils::constants::DATABASE_URL).clone(); // clone() is used to copy
    let db: DatabaseConnection = Database::connect(database_url)
    .await
    .map_err(|err| MainError { message: err.to_string() })?;

    Migrator::up(&db, None)
    .await
    .map_err(|err| MainError { message: err.to_string() })?; // for migration

    HttpServer::new(move || {
        App::new()
        .app_data(web::Data::new(AppState { db: db.clone() })) // to connect to db
        .wrap(Logger::default())
        .configure(routes::home_routes::config) // routes
        .configure(routes::auth_routes::config)
        .configure(routes::user_routes::config)
    })
    .bind((address, port))
    .map_err(|err| MainError { message: err.to_string() })?
    .run()
    .await
    .map_err(|err| MainError { message: err.to_string() })
}








// use actix_web::{get, web, App, HttpServer, Responder, HttpResponse};
// use std::io;

// // Add error handling and logging
// #[get("/hello/{name}")]
// async fn greet(name: web::Path<String>) -> impl Responder {
//     // Sanitize the input and handle potential edge cases
//     let sanitized_name = name.trim();
//     if sanitized_name.is_empty() {
//         return HttpResponse::BadRequest().body("Name cannot be empty");
//     }
    
//     // Return a proper HTTP response
//     HttpResponse::Ok()
//         .content_type("text/plain")
//         .body(format!("Hello {}!", sanitized_name))
// }

// // Improved main function with better error handling
// #[actix_web::main]
// async fn main() -> io::Result<()> {
//     // Initialize logging
//     env_logger::init();
    
//     println!("Starting server at http://127.0.0.1:8080");
    
//     HttpServer::new(|| {
//         App::new()
//             .service(greet)
//             // Add default service for unmatched routes
//             .default_service(web::route().to(|| async { "404 Not Found" }))
//     })
//     .bind(("127.0.0.1", 8080))
//     .map(|res| {
//         res.map_err(|e| {
//             eprintln!("Server error: {}", e);
//             e
//         })
//     })?
//     .run()
//     .await
// }


// fn main() {
//     println!("Hello, world!");
// }