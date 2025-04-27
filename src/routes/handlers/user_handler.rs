use actix_web::{get, web, patch};
use sea_orm::{ActiveValue::Set, EntityTrait, IntoActiveModel, ActiveModelTrait};
use serde::{Deserialize, Serialize};

use crate::utils::{api_response::{self, ApiResponse}, app_state, jwt::Claims};

#[derive(Serialize, Deserialize)] 
struct UpdateUserModel {
    name: String
}

#[get("")]
pub async fn user(app_state: web::Data<app_state::AppState>, claim_data: Claims) -> Result<ApiResponse, ApiResponse> {
    let user_model = entity::user::Entity::find_by_id(claim_data.id)
    .one(&app_state.db).await.map_err(|err| ApiResponse::new(500, err.to_string()))?
    .ok_or(ApiResponse::new(404, "user not found".to_owned()))?;
    Ok(api_response::ApiResponse::new(200, format!("{{'name': '{}', 'email': '{}'}}", user_model.name, user_model.email)))
}

#[patch("/update")]
pub async fn update_user(
    app_state: web::Data<app_state::AppState>, 
    user_data:web::Json<UpdateUserModel>, 
    claim_data: Claims
) -> Result<ApiResponse, ApiResponse> {
    let mut user_model = entity::user::Entity::find_by_id(claim_data.id).one(&app_state.db).await
    .map_err(|err| ApiResponse::new(500, err.to_string()))?
    .ok_or(ApiResponse::new(404, "user not found".to_owned()))?
    .into_active_model();

    user_model.name = Set(user_data.name.clone());
    user_model.update(&app_state.db).await
    .map_err(|err| ApiResponse::new(400, err.to_string()))?;

    Ok(ApiResponse::new(200, "user data updated".to_string()))
}