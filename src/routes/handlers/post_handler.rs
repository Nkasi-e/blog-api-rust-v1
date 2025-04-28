use actix_multipart::form::{tempfile::TempFile, MultipartForm, text::Text};
use actix_web::{post, web, get};
use chrono::{NaiveDateTime, Utc};
use sea_orm::{ActiveValue::Set, EntityTrait, ActiveModelTrait, QueryFilter, ColumnTrait};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::utils::{self, api_response::ApiResponse, app_state, jwt::Claims};


#[derive(Serialize, Deserialize)]
struct CreatePostModelSchema{
    title: String,
    text: String,
    image: String,
}


// #[derive(MultipartForm)]
// struct CreatePostModelSchema{
//     title: Text<String>,
//     text: Text<String>,
//     image: TempFile,
// }

#[derive(Serialize, Deserialize)]
struct GetPostModelSchema{
    pub id: i32,
    pub title: String,
    pub text: String,
    pub uuid: Uuid,
    pub image: String,
    pub user_id: i32,
    pub created_at: NaiveDateTime,
    pub user: Option<UserModel>
}

#[derive(Serialize, Deserialize)]
struct UserModel {
    name: String,
    email: String
}


#[post("/create")]
pub async fn create_post(
    app_state: web::Data<app_state::AppState>, 
    claim_data: Claims, 
    post_model: web::Json<CreatePostModelSchema>
    // post_model: MultipartForm<CreatePostModelSchema> // to accept image upload
) -> Result<ApiResponse, ApiResponse> {


    // let check_name = post_model.file.file_name.clone().unwrap_or("null".to_owned());
    // let max_file_size = (*utils::constants::MAX_FILE_SIZE).clone();

    // match &check_name[check_name.len() - 4 ..] {
    //     ".png" | ".jpg" => {},
    //     _ => {
    //         return  Err(ApiResponse::new(401, "Invalid file type".to_owned()))
    //     }
    // }

    // match post_model.file.size {
    //     0 => {
    //         return  Err(ApiResponse::new(401, "Invalid file type".to_owned()))
    //     },
    //     length: if length > max_file_size => {
    //         return  Err(ApiResponse::new(401, "file too big".to_owned()))
    //     }
    // }

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
        uuid: post.uuid,
        user: None
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
        uuid: post.uuid,
        user: None
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
        .find_also_related(entity::user::Entity) // Left Join to connect the post and get the user information
        .one(&app_state.db)
        .await
        .map_err(|err| ApiResponse::new(500, err.to_string()))? // Handle DB errors

    // Convert result into GetPostModelSchema and map it to ApiResponse (because of the join that returns a tupple we add 0)
    .map(|post| GetPostModelSchema {
        id: post.0.id,
        user_id: post.0.user_id,
        created_at: post.0.created_at,
        image: post.0.image,
        text: post.0.text,
        title: post.0.title,
        uuid: post.0.uuid,
        user: post.1.map(|item| UserModel { name: item.name, email: item.email })
    })
    .ok_or_else(|| ApiResponse::new(404, "No post found".to_string()))?; // Handle empty result

    // Convert the GetPostModelSchema into a JSON string
    let res_str = serde_json::to_string(&post)
        .map_err(|err| ApiResponse::new(500, err.to_string()))?;

    Ok(ApiResponse::new(200, res_str))
}


