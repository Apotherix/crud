use axum::{
    routing::{get, post},
    Router,
    Extension,
};

use crate::db::pool::init_pool;
use crate::routes::user::{create_user, get_user, update_user, delete_user};

pub async fn build_router() -> axum::Router {
    let pool = init_pool().await.expect("Failed to create DB pool");

    Router::new()
        .route("/users", post(create_user))
        .route("/users/id", get(get_user). put(update_user). delete(delete_user))
    .layer(Extension(pool))
}

pub mod user;