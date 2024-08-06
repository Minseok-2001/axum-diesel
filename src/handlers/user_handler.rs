use std::sync::Arc;

use axum::{
    extract::{Extension, Path},
    Json,
};
use serde::Deserialize;
use tokio::sync::Mutex;

use crate::domain::models::user::User;
use crate::infra::db::DbPool;
use crate::infra::repositories::user_repository;

pub async fn create_user(
    Extension(pool): Extension<Arc<Mutex<DbPool>>>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<User>, String> {
    let mut conn = pool.lock().await.get().map_err(|e| e.to_string())?;
    let user = user_repository::create_user(&mut conn, &payload.name).map_err(|e| e.to_string())?;
    Ok(Json(user))
}

pub async fn get_user(
    Extension(pool): Extension<Arc<Mutex<DbPool>>>,
    Path(user_id): Path<i32>,
) -> Result<Json<User>, String> {
    let mut conn = pool.lock().await.get().map_err(|e| e.to_string())?;
    let user = user_repository::get_user(&mut conn, user_id).map_err(|e| e.to_string())?;
    Ok(Json(user))
}

pub async fn update_user(
    Extension(pool): Extension<Arc<Mutex<DbPool>>>,
    Path(user_id): Path<i32>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<Json<User>, String> {
    let mut conn = pool.lock().await.get().map_err(|e| e.to_string())?;
    let user = user_repository::update_user(&mut conn, user_id, &payload.name).map_err(|e| e.to_string())?;
    Ok(Json(user))
}

pub async fn delete_user(
    Extension(pool): Extension<Arc<Mutex<DbPool>>>,
    Path(user_id): Path<i32>,
) -> Result<Json<usize>, String> {
    let mut conn = pool.lock().await.get().map_err(|e| e.to_string())?;
    let result = user_repository::delete_user(&mut conn, user_id).map_err(|e| e.to_string())?;
    Ok(Json(result))
}

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub name: String,
}

#[derive(Deserialize)]
pub struct UpdateUserRequest {
    pub name: String,
}
