use actix_web::{post, web, get};
use chrono::{NaiveDateTime, Utc};
use sea_orm::{ActiveValue::Set, EntityTrait, ActiveModelTrait, QueryFilter, ColumnTrait};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::utils::{api_response::ApiResponse, app_state, jwt::Claims};


#[derive(Serialize, Deserialize)]
struct CreatePostModelSchema{
    title: String,
    text: String,
    image: String,
}

#[derive(Serialize, Deserialize)]
struct GetPostModelSchema{
    pub id: i32,
    pub title: String,
    pub text: String,
    pub uuid: Uuid,
    pub image: String,
    pub user_id: i32,
    pub created_at: NaiveDateTime,
}


#[post("/create")]
pub async fn create_post(
    app_state: web::Data<app_state::AppState>, 
    claim_data: Claims, 
    post_model: web::Json<CreatePostModelSchema>
) -> Result<ApiResponse, ApiResponse> {

    let post_entity = entity::post::ActiveModel {
        title: Set(post_model.title.clone()),
        text: Set(post_model.text.clone()),
        image: Set(post_model.image.clone()),
        uuid: Set(Uuid::new_v4()),
        user_id: Set(claim_data.id),
        created_at: Set(Utc::now().naive_local()),
        ..Default::default()
    };

    post_entity.insert(&app_state.db).await
    .map_err(|err | ApiResponse::new(500, err.to_string()))?;

    Ok(ApiResponse::new(201, "Post successfully created".to_owned()))
}

#[get("/my-post")]
pub async fn my_post(
    app_state: web::Data<app_state::AppState>,
     claim_data: Claims
) -> Result<ApiResponse, ApiResponse> {
    let post: Vec<GetPostModelSchema> = entity::post::Entity::find()
    .filter(entity::post::Column::UserId.eq(claim_data.id))
    .all(&app_state.db).await
    .map_err(|err | ApiResponse::new(500, err.to_string()))?
    .into_iter().map(|post| GetPostModelSchema {
        id: post.id,
        user_id: post.user_id,
        created_at: post.created_at,
        image: post.image,
        text: post.text,
        title: post.title,
        uuid: post.uuid
    }).collect();

    let res_str = serde_json::to_string(&post)
    .map_err(|err | ApiResponse::new(500, err.to_string()))?;

    Ok(ApiResponse::new(200, res_str.to_owned()))
}

#[get("/all-post")]
pub async fn all_post(
    app_state: web::Data<app_state::AppState>,
    //  claim_data: Claims
) -> Result<ApiResponse, ApiResponse> {
    let post: Vec<GetPostModelSchema> = entity::post::Entity::find()
    .all(&app_state.db).await
    .map_err(|err | ApiResponse::new(500, err.to_string()))?
    .into_iter().map(|post| GetPostModelSchema {
        id: post.id,
        user_id: post.user_id,
        created_at: post.created_at,
        image: post.image,
        text: post.text,
        title: post.title,
        uuid: post.uuid
    }).collect();


    let res_str = serde_json::to_string(&post)
    .map_err(|err | ApiResponse::new(500, err.to_string()))?;

    Ok(ApiResponse::new(200, res_str.to_owned()))
}


#[get("/single-post/{post_uuid}")]
pub async fn single_post(
    app_state: web::Data<app_state::AppState>,
    post_uuid: web::Path<Uuid>, // to get params
    // claim_data: Claims,
) -> Result<ApiResponse, ApiResponse> {
    let post = entity::post::Entity::find()
        .filter(entity::post::Column::Uuid.eq(post_uuid.clone()))
        .one(&app_state.db)
        .await
        .map_err(|err| ApiResponse::new(500, err.to_string()))? // Handle DB errors

    // Convert result into GetPostModelSchema and map it to ApiResponse
    .map(|post| GetPostModelSchema {
        id: post.id,
        user_id: post.user_id,
        created_at: post.created_at,
        image: post.image,
        text: post.text,
        title: post.title,
        uuid: post.uuid,
    })
    .ok_or_else(|| ApiResponse::new(404, "No post found".to_string()))?; // Handle empty result

    // Convert the GetPostModelSchema into a JSON string
    let res_str = serde_json::to_string(&post)
        .map_err(|err| ApiResponse::new(500, err.to_string()))?;

    Ok(ApiResponse::new(200, res_str))
}
