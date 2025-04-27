use sea_orm::DatabaseConnection;


// for db connection
pub struct AppState {
    pub db: DatabaseConnection
}