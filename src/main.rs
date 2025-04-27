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
        .configure(routes::post_routes::config)
    })
    .bind((address, port))
    .map_err(|err| MainError { message: err.to_string() })?
    .run()
    .await
    .map_err(|err| MainError { message: err.to_string() })
}
