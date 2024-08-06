use axum::{
    routing::{get, post, put, delete},
    Router,
};
use crate::handlers::user_handler::{create_user, get_user, update_user, delete_user};
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::infra::db::DbPool;

pub fn create_routes(pool: Arc<Mutex<DbPool>>) -> Router {
    Router::new()
        .route("/users", post(create_user))
        .route("/users/:id", get(get_user))
        .route("/users/:id", put(update_user))
        .route("/users/:id", delete(delete_user))
        .layer(axum::extract::Extension(pool))
}
