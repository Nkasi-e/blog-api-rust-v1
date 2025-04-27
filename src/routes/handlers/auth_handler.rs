
use actix_web::{post, web};
use sea_orm::{ActiveValue::Set, EntityTrait};
use sea_orm::{ActiveModelTrait, QueryFilter, Condition, ColumnTrait};
use sha256::digest;
use crate::utils::api_response::ApiResponse;
use crate::utils::jwt::encode_jwt;
use crate::utils::{api_response, app_state};
use serde::{Deserialize, Serialize};



#[derive(Serialize, Deserialize)] 
struct RegisterModel {
    name: String,
    email: String,
    password: String
}

#[derive(Serialize, Deserialize)]
struct LoginModel {
    email: String,
    password: String
}
// app_state is used to get the database connection

#[post("/register")]
pub async fn register(app_state: web::Data<app_state::AppState>, 
    register_json: web::Json<RegisterModel>) -> Result<ApiResponse, ApiResponse> {
    let user_model = entity::user::ActiveModel {
        name: Set(register_json.name.clone()),
        email: Set(register_json.email.clone()),
        password: Set(digest(&register_json.password)), // using digest from sha256 to hash the password before saving
        ..Default::default()
    }.insert(&app_state.db).await
    .map_err(|err| ApiResponse::new(500, err.to_string()))?;

    Ok(api_response::ApiResponse::new(201, format!("{}", user_model.id)))
}


#[post("/login")]
pub async fn login(app_state: web::Data<app_state::AppState>, 
    login_json: web::Json<LoginModel>) -> Result<ApiResponse, ApiResponse> {

    let user = entity::user::Entity::find().filter(Condition::all()
    .add(entity::user::Column::Email.eq(&login_json.email))
    .add(entity::user::Column::Password.eq(digest(&login_json.password)))
).one(&app_state.db).await
.map_err(|err| ApiResponse::new(500, err.to_string()))?
.ok_or(ApiResponse::new(404, "user not found".to_owned()))?;



    let token = encode_jwt(user.email, user.id).map_err(|err| ApiResponse::new(422, err.to_string()))?;

    Ok(api_response::ApiResponse::new(200, format!("{{ 'token': '{}' }}", token))) // singular curly braces are used for passing data values
}